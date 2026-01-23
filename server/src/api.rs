use crate::AppState;
use crate::auth::AuthUser;
use crate::models::{Alarm, User};
use crate::services::database::DBError;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router, http::StatusCode};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<DBError> for ApiError {
    fn from(error: DBError) -> Self {
        match error {
            DBError::UserNotFound => ApiError::NotFound("Utilisateur non trouvé".into()),
            DBError::UserAlreadyExists(email) => {
                ApiError::BadRequest(format!("L'utilisateur {} existe déjà", email))
            }
            _ => ApiError::Internal(error.to_string()),
        }
    }
}

pub fn create_router(state: AppState) -> Router {
    let mut api_router = Router::new()
        .route("/calendar", get(get_calendar))
        .route("/health", get(health_check))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/register-device", post(register_device))
        .route("/alarms", post(update_alarms))
        .route("/user", get(get_user));

    if cfg!(feature = "local_dev") {
        api_router = api_router.route("/test-notification", get(test_notification))
    }

    Router::new()
        .nest("/viva-padel", api_router)
        .layer(TraceLayer::new_for_http())
        .layer(axum::Extension(state.clone()))
        .with_state(state)
}

pub(crate) async fn get_calendar(State(state): State<AppState>) -> Json<crate::Calendar> {
    let cal = state.calendar.read().expect("Failed to read calendar");
    Json(cal.clone())
}

pub(crate) async fn health_check() -> &'static str {
    "OK"
}

#[derive(Deserialize)]
pub struct RegisterDeviceRequest {
    pub notif_token: String,
    pub device_id: String,
}

pub(crate) async fn register_device(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<RegisterDeviceRequest>,
) -> Result<StatusCode, ApiError> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|e| ApiError::BadRequest(format!("ID utilisateur invalide: {}", e)))?;

    // Insert or update device
    state
        .db
        .register_device(&payload.device_id, &payload.notif_token, user_id)
        .await?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct UpdateAlarmRequest {
    pub alarms: Vec<Alarm>,
}

pub(crate) async fn update_alarms(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<UpdateAlarmRequest>,
) -> Result<StatusCode, ApiError> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|e| ApiError::BadRequest(format!("Invalid user ID: {}", e)))?;

    // Update user alarms
    state.db.update_alarms(user_id, payload.alarms).await?;

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user: User,
    pub alarms: Vec<Alarm>,
}

pub(crate) async fn get_user(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<GetUserResponse>, ApiError> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|e| ApiError::BadRequest(format!("Invalid user ID: {}", e)))?;

    let user = state.db.get_user_by_id(user_id).await?;

    // Fetch alarms
    let alarms = state.db.get_user_alarms(user_id).await?;

    let response = GetUserResponse { user, alarms };
    Ok(Json(response))
}

#[derive(Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(email)]
    pub email: String,
}

pub(crate) async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    let user = state.db.create_user(&payload.email).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub(crate) async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), ApiError> {
    use crate::auth::Claims;
    let user = state.db.get_user_by_email(&payload.email).await?;

    let expiration = Utc::now()
        .checked_add_signed(TimeDelta::days(14))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        iat: Utc::now().timestamp() as usize,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(LoginResponse { token })))
}

#[cfg(feature = "local_dev")]
#[derive(Deserialize)]
pub struct TestNotifRequest {
    pub device_token: String,
    pub title: Option<String>,
    pub message: Option<String>,
}

#[cfg(feature = "local_dev")]
pub(crate) async fn test_notification(
    State(state): State<AppState>,
    Json(payload): Json<TestNotifRequest>,
) -> Result<StatusCode, ApiError> {
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    use crate::services::legarden::DATE_FORMAT;
    use chrono::Datelike;
    use chrono::Duration;

    let today = chrono::Local::now().date_naive();
    let monday_next_week =
        today + Duration::days((7 - today.weekday().num_days_from_monday() as i64) % 7);
    let today = today.format(DATE_FORMAT).to_string();
    let monday_next_week = monday_next_week.format(DATE_FORMAT).to_string();

    let simple_day = crate::models::legarden::DayPlanningResponse::simple_day();
    let mut avail = BTreeMap::new();
    avail.insert(today, simple_day.clone());
    avail.insert(monday_next_week, simple_day);

    let mut final_avail = HashMap::new();
    final_avail.insert("Mon alarme préférée".to_owned(), avail);
    let data = Some(serde_json::json!({ "availabilities": final_avail}));
    let title = payload.title.as_deref().unwrap_or("Test Notification 🎾");
    let message = payload
        .message
        .as_deref()
        .unwrap_or("Ceci est un test de Viva Padel !");

    if let Err(e) = state
        .notifications
        .send_notification(&[payload.device_token], title, message, data)
        .await
    {
        return Err(ApiError::Internal(e));
    }

    Ok(StatusCode::OK)
}
