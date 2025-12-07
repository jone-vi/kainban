use axum::Router;
use sqlx::MySqlPool;

pub mod agents;
pub mod jobs;
pub mod healthz;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .nest("/agents", agents::routes(pool.clone()))
        .nest("/jobs", jobs::routes(pool.clone()))
        .nest("/healthz", healthz::routes())
}
