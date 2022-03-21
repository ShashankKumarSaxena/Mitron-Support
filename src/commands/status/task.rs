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

        let response = reqwest::get(request_url).await.unwrap();
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
                    e.description(format!("{} | Ping: `{}`ms", status_emoji, status.ping));
                    e.color(0x2F3136);
                    e
                })
            })
            .await;
    }
}
