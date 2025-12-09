use axum::Router;
use sqlx::MySqlPool;

pub mod agents;
pub mod jobs;
pub mod healthz;
pub mod users;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .nest("/agents", agents::routes())
        .nest("/jobs", jobs::routes())
        .nest("/users", users::routes())
        .nest("/healthz", healthz::routes())
        .with_state(pool)
}
