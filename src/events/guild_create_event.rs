use crate::utils::typemaps::PgConnectionPool;
use serenity::{model::prelude::*, prelude::*};
use tracing::info;

pub async fn guild_create(ctx: Context, guild: Guild, is_new: bool) -> Result<(), sqlx::Error> {
    let pool = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    sqlx::query("INSERT INTO guildconfig (id, welcome_channel_id) VALUES ($1, NULL) ON CONFLICT DO NOTHING;").bind(guild.id.0 as i64).execute(&pool).await?;

    if is_new {
        info!("[GUILD_CREATE] New guild created: {}", guild.name);
    }

    Ok(())
}
