use axum::{extract::State, routing::post, Json, Router};
use sqlx::MySqlPool;

use crate::models::user::{CreateUser, UserWithToken, User};

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/register", post(register))
}

async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<UserWithToken>, (axum::http::StatusCode, String)> {
    match User::create(&pool, payload).await {
        Ok(user_with_token) => Ok(Json(user_with_token)),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to create user: {}", err),
        )),
    }
}
