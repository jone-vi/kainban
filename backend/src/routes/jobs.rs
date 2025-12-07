use axum::{routing::post, Json, Router};
use sqlx::MySqlPool;

use crate::models::job::{CreateJob, Job};

pub fn routes(pool: MySqlPool) -> Router {
    Router::new().route("/", post(move |body| create_job(pool.clone(), body)))
}

async fn create_job(pool: MySqlPool, Json(payload): Json<CreateJob>) -> Result<Json<Job>, Json<String>> {
    match Job::create(&pool, payload).await {
        Ok(job) => Ok(Json(job)),
        Err(err) => Err(Json(format!("failed to create job: {}", err))),
    }
}
