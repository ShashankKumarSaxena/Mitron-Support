use crate::commands::status::botstatus::BotStatus;
use crate::utils::typemaps::PgConnectionPool;
use serde::{Deserialize, Serialize};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::env;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub async fn reset_bot_status(ctx: Arc<Context>) {
    let pool = &ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let watcher_data = sqlx::query!("SELECT watcher_channel, watcher_message FROM guildconfig;")
        .fetch_all(pool)
        .await
        .unwrap();

    for data in watcher_data.iter() {
        let channel_id = match data.watcher_channel {
            Some(v) => v,
            None => continue,
        };

        let message_id = match data.watcher_message {
            Some(v) => v,
            None => continue,
        };

        let channel = ctx.http.get_channel(channel_id as u64).await.unwrap();
        let mut message = ctx
            .http
            .get_message(channel_id as u64, message_id as u64)
            .await
            .unwrap();

        let request_url = env::var("STATUS_API").unwrap();

        let response = reqwest::get(request_url).await;

        match response {
            Ok(response) => {
                let status: BotStatus = response.json::<BotStatus>().await.unwrap();
                let mut status_emoji: &str;

                if status.status == "alive" {
                    status_emoji = "🟢 | Online";
                } else {
                    status_emoji = "🔴 | Offline";
                }

                message
                    .edit(&ctx.http, |m| {
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
                    .await;
            }
            Err(_) => {
                message
                    .edit(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Miत्रों Status");
                            e.description("```\n🔴 | Offline\n```\n*Bot is currently offline. Please keep patience till the developers make it back on.*");
                            e.color(0x2F3136);
                            e
                        })
                    })
                    .await;
            }
        }
    }
}
