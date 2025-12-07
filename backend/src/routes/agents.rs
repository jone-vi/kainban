use axum::{Json, Router, routing::post};
use sqlx::MySqlPool;
use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterAgent {
    name: String,
}

pub fn routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/register", post(move |body| register(pool.clone(), body)))
}

async fn register(pool: MySqlPool, Json(payload): Json<RegisterAgent>) -> Json<String> {
    let id = uuid::Uuid::now_v7();
    let now = chrono::Utc::now();


    Json(format!("registered agent {}", id))
}
