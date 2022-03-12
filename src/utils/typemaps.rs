use serenity::prelude::TypeMapKey;
use sqlx::PgPool;

pub struct PgConnectionPool;

impl TypeMapKey for PgConnectionPool {
    type Value = PgPool;
}