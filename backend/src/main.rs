use std::net::SocketAddr;

use axum;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener;

mod config;
mod auth;
mod db;
mod token;
mod models;
mod routes;

use crate::config::Config;
use crate::db::create_pool;
use crate::routes::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env()?;
    let pool = create_pool(&cfg.database_url).await?;

    let app = create_router(pool.clone());

    let addr: SocketAddr = cfg.server_addr.parse().unwrap();
    tracing::info!("Backend running on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
