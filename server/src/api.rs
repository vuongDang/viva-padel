use crate::AppState;
use crate::auth::AuthUser;
use crate::models::{Alarm, Device, NotifInfo, User};
use crate::services::database::DBError;
use axum::extract::State;
use axum::http::HeaderValue;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router, http::StatusCode};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tower_http::cors::CorsLayer;
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
            DBError::Sqlx(sqlx::Error::Database(err)) if err.code() == Some("787".into()) => {
                ApiError::Unauthorized("Session invalide ou utilisateur supprimé (FK error)".into())
            }
            _ => ApiError::Internal(error.to_string()),
        }
    }
}

pub fn create_router(state: AppState) -> Router {
    dotenvy::dotenv().ok();
    let pwa_url = std::env::var("PWA_URL").expect("PWA_URL must be set");
    tracing::info!("PWA_URL: {}", &pwa_url);
    let origins = [
        pwa_url.parse::<HeaderValue>().unwrap(),
        "https://viva-padel-app.xoi-lap-xuong.com"
            .parse::<HeaderValue>()
            .unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        // .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        // .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .allow_credentials(true);

    tracing::info!("CORS: {:?}", cors);

    Router::new()
        .route("/calendar", get(get_calendar))
        .route("/health", get(health_check))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/register-device", post(register_device))
        .route("/alarms", post(update_alarms))
        .route("/user", get(get_user))
        .route("/test-notification", get(test_notification))
        .layer(TraceLayer::new_for_http())
        .layer(axum::Extension(state.clone()))
        .layer(cors)
        .with_state(state)
}

pub(crate) async fn get_calendar(State(state): State<AppState>) -> Json<crate::Calendar> {
    let cal = state.calendar.read().expect("Failed to read calendar");
    Json(cal.clone())
}

#[derive(Serialize)]
pub struct HealthResponse {
    name: &'static str,
    version: &'static str,
}
pub(crate) async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub(crate) async fn register_device(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<Device>,
) -> Result<StatusCode, ApiError> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|e| ApiError::BadRequest(format!("ID utilisateur invalide: {}", e)))?;
    match payload.notif_info {
        NotifInfo::Mobile(notif_token) => {
            // Insert or update device
            state
                .db
                .register_mobile(&payload.device_id, &notif_token, user_id)
                .await?;
        }
        NotifInfo::Web(info) => {
            state
                .db
                .register_browser(info, user_id, &payload.device_id)
                .await?;
        }
    }

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Debug)]
pub struct UpdateAlarmRequest {
    pub alarms: Vec<Alarm>,
}

#[tracing::instrument(name = "Update alarms", skip(auth, state))]
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
    pub devices: Vec<Device>,
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

    // Fetch devices
    let devices = state.db.get_devices_for_user(user_id).await?;

    let response = GetUserResponse {
        user,
        alarms,
        devices,
    };
    Ok(Json(response))
}

#[derive(Deserialize, Validate, Debug)]
pub struct SignupRequest {
    #[validate(email)]
    pub email: String,
}

#[tracing::instrument(name = "Signing up", skip(state))]
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

    // 6 months expiration date
    let expiration = Utc::now()
        .checked_add_signed(TimeDelta::days(180))
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

#[derive(Deserialize)]
pub struct TestNotifRequest {
    pub user_id: Uuid,
    pub title: Option<String>,
    pub message: Option<String>,
}

pub(crate) async fn test_notification(
    State(state): State<AppState>,
    Json(payload): Json<TestNotifRequest>,
) -> Result<StatusCode, ApiError> {
    let devices = state.db.get_devices_for_user(payload.user_id).await?;

    let title = payload.title.as_deref().unwrap_or("Test Notification 🎾");
    let message = payload
        .message
        .as_deref()
        .unwrap_or("Ceci est un test de Viva Padel !");

    if let Err(e) = state
        .notifications
        .send_notification(&devices, title, message, None)
        .await
    {
        return Err(ApiError::Internal(e));
    }

    Ok(StatusCode::OK)
}
