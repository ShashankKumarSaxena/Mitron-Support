use crate::utils::typemaps::PgConnectionPool;
use serenity::{model::gateway::Activity, model::guild::Member, model::prelude::*, prelude::*};
use sqlx;
use sqlx::Row;

pub async fn guild_member_addition(
    ctx: Context,
    guild_id: GuildId,
    member: Member,
) -> Result<(), sqlx::Error> {
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
    println!("{}", channel_id);

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

        let mcur = sqlx::query("SELECT welcome_message FROM guildconfig WHERE id = $1")
            .bind(guild_id.0 as i64)
            .fetch_one(&db)
            .await
            .unwrap();

        let welcome_msg = match mcur.try_get::<&str, _>("welcome_message") {
            Ok(value) => { value.replace("<<member>>", format!("<@{}>", member.user.id.0).as_str()) },
            Err(_) => format!("Hello {}, welcome to this server. Hope you have a great time here. Please check out rules channel first.", member.display_name()),
        };

        channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("Welcome to {}!", guild_name).as_str());
                    e.description(welcome_msg);
                    e
                })
            })
            .await;

        return Ok(());
    }

    Ok(())
}
