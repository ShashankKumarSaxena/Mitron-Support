use sqlx::postgres::{PgPool, PgPoolOptions};
use std::fs;

// Function to get postgres pool connection.
pub async fn get_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    return Ok(PgPoolOptions::new()
        .max_connections(50)
        .connect(db_url)
        .await?);
}

pub async fn execute_queries(pool: PgPool) -> Result<(), sqlx::Error> {
    let queries =
        fs::read_to_string("../database/schema.sql").expect("[DATABASE] Schema is not present!");

    sqlx::query(queries.as_str()).execute(&pool).await?;
    Ok(())
}
