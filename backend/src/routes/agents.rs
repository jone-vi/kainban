use axum::{Json, 
    Router, 
    routing::post, 
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::MySqlPool;
use serde::Deserialize;
use tower_http::trace::TraceLayer;

use crate::models::agent::{Agent, CreateAgent};

pub fn routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/register", post(move |body| register(pool.clone(), body))).layer(TraceLayer::new_for_http())
}

async fn register(pool: MySqlPool, Json(payload): Json<CreateAgent>) -> impl IntoResponse {

    let new = CreateAgent {
        name: payload.name,
        workspace_volume: payload.workspace_volume,
    };

    match Agent::create_agent(&pool, new).await {
        Ok(agent) => {
            tracing::info!(%agent.id, "Agent successfully created");
            Json(agent).into_response()
        }
        Err(err) => {
            tracing::error!(?err, "Failed to create agent");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to register agent",
            ).into_response()
        }
    };

}

async fn get_all_agents(pool: MySqlPool) -> impl IntoResponse {

    match Agent::get_all_agents(&pool).await {
        Ok(agents) => {
            tracing::info!(count = agents.len(), "Fetched agents");
            Json(agents).into_response()
        }
        Err(err) => {
            tracing::error!(?err, "Failed to get all agents");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get all agents",
            ).into_response()
        }
    }
}
