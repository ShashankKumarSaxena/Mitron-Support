use crate::utils::typemaps::PgConnectionPool;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use sqlx::Row;
use tracing::warn;

#[command]
#[description("Set up welcome messages.")]
#[usage("<channel>")]
#[example("#welcome")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn welcome(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.mention_channels.len() == 0 {
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

    let channel = &msg.mention_channels[0];

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
                .bind(channel.id.0 as i64)
                .bind(guild_id.0 as i64)
                .execute(&db)
                .await?;

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(
                            format!(
                                "Successfully set the welcome channel to <#{}>",
                                channel.id.0
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
