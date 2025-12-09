use axum::{routing::get, Router};
use sqlx::MySqlPool;

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/", get(|| async { "OK" }))
}
