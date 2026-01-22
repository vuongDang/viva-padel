use crate::AppState;
use crate::auth::AuthUser;
use crate::models::{Alarm, User};
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
    sqlx::query!(
        "INSERT INTO devices (device_id, notif_token, user_id, last_seen) VALUES (?, ?, ?, CURRENT_TIMESTAMP)
        ON CONFLICT(device_id)
        DO UPDATE SET
        user_id = excluded.user_id,
        last_seen = CURRENT_TIMESTAMP,
        notif_token = excluded.notif_token",
        payload.device_id,
        payload.notif_token,
        user_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

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

    // Remove all previous user alarms
    sqlx::query!("DELETE FROM alarms WHERE user_id = ?", user_id)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let now = Utc::now();
    for alarm in payload.alarms {
        let alarm_id = Uuid::new_v4();
        let alarm_json = serde_json::json!(alarm).to_string();
        sqlx::query!(
            "INSERT INTO alarms (id, user_id, alarm_json, is_active, created_at) VALUES (?, ?, ?, ?, ?)",
            alarm_id,
            user_id,
            alarm_json,
            alarm.is_active,
            now
        )
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    }

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

    let user = sqlx::query_as::<_, User>("SELECT * FROM users where id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Fetch alarms
    let alarms_rows = sqlx::query!("SELECT alarm_json FROM alarms WHERE user_id = ?", user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let alarms: Vec<Alarm> = alarms_rows
        .into_iter()
        .map(|record| {
            let alarm_json: String = record.alarm_json;
            serde_json::from_str::<Alarm>(&alarm_json)
                .map_err(|e| ApiError::Internal(e.to_string()))
        })
        .collect::<Result<Vec<Alarm>, ApiError>>()?;

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

    let user = User::new(payload.email);
    sqlx::query!(
        "INSERT INTO users (id, email, created_at) VALUES (?, ?,  ?)",
        user.id,
        user.email,
        user.created_at
    )
    .execute(&state.db)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

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
    let user = sqlx::query_as::<_, User>("SELECT * FROM users where email = ?")
        .bind(&payload.email)
        .fetch_one(&state.db)
        .await
        .map_err(|_| ApiError::BadRequest(format!("Utilisateur non trouvé: {}", payload.email)))?;

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
    State(_state): State<AppState>,
    Json(payload): Json<TestNotifRequest>,
) -> Result<StatusCode, ApiError> {
    let title = payload.title.as_deref().unwrap_or("Test Notification 🎾");
    let message = payload
        .message
        .as_deref()
        .unwrap_or("Ceci est un test de Viva Padel !");

    if let Err(e) =
        crate::send_push_notification(&[payload.device_token], title, message, None).await
    {
        return Err(ApiError::Internal(e));
    }

    Ok(StatusCode::OK)
}
