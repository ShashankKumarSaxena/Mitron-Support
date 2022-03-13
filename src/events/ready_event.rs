use crate::db::execute_queries;
use crate::utils::typemaps::PgConnectionPool;
use serenity::{model::gateway::Activity, model::prelude::*, prelude::*};
use tracing::{error, info};

pub async fn ready(ctx: Context, ready: Ready) {
    ctx.set_activity(Activity::watching("Miत्रों Support Server"))
        .await;

    let pool = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    execute_queries(pool).await;

    info!("[CORE] {} is connected!", ready.user.name);
}
