use crate::config::Config;
use sqlx::{SqlitePool, Error};

pub async fn create_pool(config: &Config) -> Result<SqlitePool, Error> {
    SqlitePool::connect(config.database_url.as_str()).await
}