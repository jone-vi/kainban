use sqlx::{Pool, MySql, mysql::MySqlPoolOptions};

pub async fn create_pool(db_url: &str) -> anyhow::Result<Pool<MySql>> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;

    Ok(pool)
}
