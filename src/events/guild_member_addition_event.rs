use crate::utils::typemaps::PgConnectionPool;
use serenity::{model::gateway::Activity, model::guild::Member, model::prelude::*, prelude::*};
use sqlx;
use sqlx::Row;

pub async fn guild_member_addition(ctx: &Context, guild_id: GuildId, member: Member) {
    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let cur = sqlx::query("SELECT welcome_channel_id FROM guildconfig WHERE id = $1")
        .bind(guild_id.0 as i64)
        .fetch_one(&db)
        .await
        .unwrap();
    let channel_id = match cur.try_get::<i64, _>("welcome_channel_id") {
        Ok(value) => value,
        Err(_) => 0,
    };

    if channel_id != 0 {
        let channel = ctx.http.get_channel(channel_id as u64).await.unwrap();
        let channel_id = channel.id();
        let guild_name = channel
            .guild()
            .unwrap()
            .guild_id
            .name(&ctx.cache)
            .await
            .unwrap();

        channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Welcome to {}!", guild_name).as_str());
                e.description("Hello {}, welcome to this server. Hope you have a great time here. Please check out rules channel first.");
                e
            })
        }).await.unwrap();
    }
}
