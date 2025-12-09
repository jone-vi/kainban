use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use sqlx::MySqlPool;

use crate::auth::AuthenticatedUser;
use crate::models::job::{CreateJob, Job};

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/", post(create_job))
}

async fn create_job(
    State(pool): State<MySqlPool>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Json(payload): Json<CreateJob>,
) -> Result<Json<Job>, Json<String>> {
    match Job::create(&pool, payload).await {
        Ok(job) => Ok(Json(job)),
        Err(err) => Err(Json(format!("failed to create job: {}", err))),
    }
}
