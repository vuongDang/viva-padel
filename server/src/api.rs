use crate::AppState;
use crate::auth::AuthUser;
use crate::models::{Alarm, User};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, http::StatusCode};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

pub fn create_router(state: AppState) -> Router {
    let api_router = Router::new()
        .route("/calendar", get(get_calendar))
        .route("/health", get(health_check))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/register-device", post(register_device))
        .route("/alarms", post(update_alarms))
        .route("/user", get(get_user));

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
    pub email: String,
    pub token: String,
}

pub(crate) async fn register_device(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDeviceRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = match sqlx::query!("SELECT id FROM users WHERE email = ?", payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        Some(row) => row.id,
        None => {
            let id = Uuid::new_v4();
            let now = Utc::now();
            sqlx::query!(
                "INSERT INTO users (id, email, created_at) VALUES (?, ?, ?)",
                id,
                payload.email,
                now
            )
            .execute(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            id.to_string()
        }
    };

    // 2. Insert or update device
    sqlx::query!(
        "INSERT INTO devices (token, user_id, last_seen) VALUES (?, ?, CURRENT_TIMESTAMP)
         ON CONFLICT(token) DO UPDATE SET user_id = excluded.user_id, last_seen = CURRENT_TIMESTAMP",
        payload.token,
        user_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid user ID: {}", e)))?;

    // Remove all previous user alarms
    sqlx::query!("DELETE FROM alarms WHERE user_id = ?", user_id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
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
) -> Result<Json<GetUserResponse>, (StatusCode, String)> {
    let user_id = Uuid::parse_str(&auth.user_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid user ID: {}", e)))?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users where id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // Fetch alarms
    let alarms_rows = sqlx::query!("SELECT alarm_json FROM alarms WHERE user_id = ?", user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let alarms: Vec<Alarm> = alarms_rows
        .into_iter()
        .map(|record| {
            let alarm_json: String = record.alarm_json;
            serde_json::from_str::<Alarm>(&alarm_json)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        })
        .collect::<Result<Vec<Alarm>, (StatusCode, String)>>()?;

    let response = GetUserResponse { user, alarms };
    Ok(Json(response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
}

pub(crate) async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user = User::new(payload.email);
    sqlx::query!(
        "INSERT INTO users (id, email, created_at) VALUES (?, ?,  ?)",
        user.id,
        user.email,
        user.created_at
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, String)> {
    use crate::auth::Claims;
    let user = sqlx::query_as::<_, User>("SELECT * FROM users where email = ?")
        .bind(payload.email)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

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
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(LoginResponse { token })))
}
