use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};
use std::env;

pub type DbPool = Pool<MySql>;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

