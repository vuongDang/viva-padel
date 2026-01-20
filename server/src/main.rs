use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use sqlx::sqlite::SqlitePool;
use viva_padel_server::{api::create_router, AppState, Calendar, poll_calendar};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().expect("Failed to load .env file");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let state = AppState {
        calendar: Arc::new(RwLock::new(Calendar::default())),
        db: pool,
        jwt_secret,
    };

    // Poll LeGarden server to get courts availabilities
    let poll_state = state.clone();
    tokio::spawn(async move {
        poll_calendar(poll_state).await;
    });

    let app = create_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
