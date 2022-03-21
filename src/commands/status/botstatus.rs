use crate::utils::typemaps::PgConnectionPool;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct BotStatus {
    pub status: String,
    pub ping: u32,
    pub shards: u32,
    pub servers: u32,
    pub avatar: String,
}

#[command]
#[description("Setup main bot's status.")]
#[usage("<channel>")]
#[example("#status")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn watcher(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.description("You need to provide a channel to set the status channel.");
                    e
                })
            })
            .await?;
        return Ok(());
    }

    let channel_id = args.single::<ChannelId>().unwrap();
    let request_url = env::var("STATUS_API").unwrap();

    let response = reqwest::get(request_url).await?;
    let status: BotStatus = response.json::<BotStatus>().await?;

    let pool = &ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let mut status_emoji: &str;

    if status.status == "alive" {
        status_emoji = "🟢 | Online";
    } else {
        status_emoji = "🔴 | Offline";
    }

    let status_msg = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Miत्रों Status");
                e.description(format!(
                    "```\n{}\n```\n**🏓 | Ping**: `{}`ms\n<:Servers:950428606755143770> **| Servers**: {}\n**🤖 | Shards**: {}",
                    status_emoji, status.ping, status.servers, status.shards
                ));
                e.color(0x2F3136);
                e.thumbnail(status.avatar);
                e
            })
        })
        .await?;

    sqlx::query!(
        "UPDATE guildconfig SET watcher_channel = $1, watcher_message = $2 WHERE id = $3;",
        channel_id.0 as i64,
        status_msg.id.0 as i64,
        msg.guild_id.unwrap().0 as i64,
    )
    .execute(pool)
    .await?;

    Ok(())
}
