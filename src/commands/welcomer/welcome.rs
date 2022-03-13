use crate::utils::typemaps::PgConnectionPool;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use sqlx::Row;
use std::i64;
use tracing::warn;

#[command]
#[description("Set up welcome messages.")]
#[usage("<channel>")]
#[example("#welcome")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn welcome(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let channel = match args.single::<ChannelId>() {
        Err(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("⚠️ Channel not provided!");
                        e.description("Please mention a channel to set up welcome messages.");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
        Ok(ch) => ch,
    };

    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    match msg.guild_id {
        Some(guild_id) => {
            sqlx::query("UPDATE guildconfig SET welcome_channel_id = $1 WHERE id = $2;")
                .bind(channel.0 as i64)
                .bind(guild_id.0 as i64)
                .execute(&db)
                .await?;

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(
                            format!(
                                "✅ Successfully set the welcome channel to <#{}>",
                                channel.0
                            )
                            .as_str(),
                        );
                        e
                    })
                })
                .await?;
            return Ok(());
        }
        None => {
            warn!("[COMMAND ERROR] Guild ID not found!")
        }
    }

    Ok(())
}

#[command("welcome-disable")]
#[description("Disable welcome messages.")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn welcome_disable(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    match msg.guild_id {
        Some(guild_id) => {
            let cursor = sqlx::query("SELECT welcome_channel_id FROM guildconfig WHERE id = $1")
                .bind(i64::from(guild_id))
                .fetch_one(&db)
                .await
                .unwrap();

            let _value = match cursor.try_get::<i64, _>("welcome_channel_id") {
                Ok(value) => value,
                Err(_) => {
                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.title("⚠️ Welcome channel not set!");
                                e.description("Please set a welcome channel first.");
                                e
                            })
                        })
                        .await?;
                    return Ok(());
                }
            };

            sqlx::query("UPDATE guildconfig SET welcome_channel_id = NULL WHERE id = $1;")
                .bind(i64::from(guild_id))
                .execute(&db)
                .await?;

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description("✅ Successfully disabled welcome messages!");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
        None => {
            warn!("[COMMAND ERROR] Guild ID not found!");
            return Ok(());
        }
    }

    Ok(())
}

#[command("welcome-message")]
#[description("Set a welcome message.")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn welcome_message(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    match msg.guild_id {
        Some(guild_id) => {
            if args.is_empty() {
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("📝 Edit Welcome Messages");
                        e.description("To edit welcome message, you need to follow the instructions given below:\n\nℹ️ **Instructions**:\n`1`: If you want that the joined member must get mentioned in the message, then add `<<member>>` in the message where you want the member to get mentioned.\n`2`: You can mention channels in the message too.");
                        e
                    })
                }).await?;
                return Ok(());
            }

            let mut message = String::new();
            while let Ok(arg) = args.single::<String>() {
                message.push_str(&arg);
                message.push_str(" ");
            }

            sqlx::query("UPDATE guildconfig SET welcome_message = $1 WHERE id = $2;")
                .bind(message)
                .bind(i64::from(guild_id))
                .execute(&db)
                .await?;

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description("✅ Successfully set the welcome message!");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
        None => {
            warn!("[COMMAND ERROR] Guild ID not found!");
            return Ok(());
        }
    }

    Ok(())
}
