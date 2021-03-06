use crate::utils::typemaps::PgConnectionPool;
use serenity::{model::gateway::Activity, model::guild::Member, model::prelude::*, prelude::*};
use sqlx;
use sqlx::Row;

// Fix multiple Queries redundancy!

pub async fn guild_member_addition(
    ctx: Context,
    guild_id: GuildId,
    mut member: Member,
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

        let icur = sqlx::query("SELECT welcome_image FROM guildconfig WHERE id = $1")
            .bind(guild_id.0 as i64)
            .fetch_one(&db)
            .await
            .unwrap();

        let img = match icur.try_get::<&str, _>("welcome_image") {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(0x2F3136);
                    e.title(format!("Welcome to {}!", guild_name).as_str());
                    e.description(welcome_msg);
                    match img {
                        Some(url) => {
                            e.image(url);
                        }
                        None => {}
                    };

                    e
                })
            })
            .await;
    }
    let rcur = match sqlx::query("SELECT autoroles FROM guildconfig WHERE id = $1")
        .bind(guild_id.0 as i64)
        .fetch_one(&db)
        .await
    {
        Ok(v) => v,
        Err(_) => {
            return Ok(());
        }
    };

    let roles = match rcur.try_get::<Vec<i64>, _>("autoroles") {
        Ok(r) => r,
        Err(_) => {
            return Ok(());
        }
    };

    if roles.len() != 0 {
        let guild_roles = ctx.http.get_guild_roles(guild_id.0 as u64).await.unwrap();
        for roleid in roles.iter() {
            let role = guild_roles.iter().find(|r| r.id.0 == *roleid as u64);
            match member.add_role(&ctx.http, role.unwrap().id).await {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    return Ok(());
}
