use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::logging::ActionLog;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> DbPool {
    let database_url = dotenvy::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn save_action_log(pool: &DbPool, log: &ActionLog) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO action_logs 
        (timestamp, action_type, user_id, ip_address, details)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        log.timestamp,
        log.action_type as _,
        log.user_id,
        log.ip_address.map(|ip| ip.to_string()),
        log.details
    )
    .execute(pool)
    .await?;
    Ok(())
}