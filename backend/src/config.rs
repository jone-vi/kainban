use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            server_addr: env::var("SERVER_ADDR").unwrap_or("0.0.0.0:8080".into()),
        })
    }
}
