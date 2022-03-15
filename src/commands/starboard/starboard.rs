use crate::utils::typemaps::PgConnectionPool;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::i64;

// This file is gonna change a lot in future. *probably but yeah it will*
//                                     - ShashankKumarSaxena

#[command("starboard")]
#[required_permissions("MANAGE_GUILD")]
#[sub_commands(enable, disable, threshold, config)]
async fn starboard(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Use `!help starboard` to see the options available.",
        )
        .await?;
    Ok(())
}

#[command]
#[required_permissions("MANAGE_GUILD")]
#[description("Enable starboard for this server.")]
#[only_in(guilds)]
async fn enable(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(0x2F3136);
                    e.title("Starboard");
                    e.description("⚠️ You need to mention a channel to enable starboard in.");
                    e
                })
            })
            .await?;
        return Ok(());
    }

    let starboard_channel = args.single::<ChannelId>().unwrap();

    let guild_id = msg.guild_id.unwrap().0;

    let pool = &ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    sqlx::query!(
        "UPDATE guildconfig SET starboard_activate = TRUE, starboard_channel = $1 WHERE id = $2",
        starboard_channel.0 as i64,
        guild_id as i64
    )
    .execute(pool)
    .await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.color(0x2F3136);
                e.title("Starboard");
                e.description(format!(
                    "Starboard successfully enabled for this server!\nStarboard Channel: <#{}>\n*To reset starboard settings, use `!starboard disable` command.*",
                    starboard_channel.0
                ));
                e
            })
        })
        .await?;

    Ok(())
}

#[command]
#[required_permissions("MANAGE_GUILD")]
#[description("Disable starboard for this server.")]
#[only_in(guilds)]
async fn disable(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().0;

    let pool = &ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    sqlx::query!(
        "UPDATE guildconfig SET starboard_activate = FALSE, starboard_channel = NULL WHERE id = $1",
        guild_id as i64
    )
    .execute(pool)
    .await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.color(0x2F3136);
                e.title("Starboard");
                e.description("Starboard successfully disabled for this server!");
                e
            })
        })
        .await?;

    Ok(())
}

#[command]
#[required_permissions("MANAGE_GUILD")]
#[description("Edit starboard star threshold.")]
#[usage("<number>")]
#[example("5")]
#[only_in(guilds)]
async fn threshold(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let pool = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let guild_data = sqlx::query!(
        "SELECT * FROM guildconfig WHERE id = $1",
        msg.guild_id.unwrap().0 as i64
    )
    .fetch_one(&pool)
    .await?;

    if guild_data.starboard_activate.unwrap() == false {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(0x2F3136);
                    e.title("Starboard");
                    e.description(
                        "Starboard is not activated! To do so, use `!starboard enable` command.",
                    );
                    e
                })
            })
            .await?;
        return Ok(());
    }

    let new_threshold = match args.single::<u32>() {
        Ok(threshold) => threshold,
        Err(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(0x2F3136);
                        e.title("Starboard");
                        e.description("Please enter a number greater than 0!");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
    };

    sqlx::query!(
        "UPDATE guildconfig SET starboard_threshold = $1 WHERE id = $2",
        new_threshold as i32,
        msg.guild_id.unwrap().0 as i64
    )
    .execute(&pool)
    .await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.color(0x2F3136);
                e.title("Starboard");
                e.description(format!(
                    "Starboard star threshold updated to {}.",
                    new_threshold
                ));
                e
            })
        })
        .await?;

    Ok(())
}

#[command]
#[required_permissions("MANAGE_GUILD")]
#[description("Check your starboard configuration.")]
#[only_in(guilds)]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let starboard_config = sqlx::query!(
        "SELECT * FROM guildconfig WHERE id = $1;",
        msg.guild_id.unwrap().0 as i64
    )
    .fetch_one(&pool)
    .await?;

    if starboard_config.starboard_activate.unwrap() == false {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(0x2F3136);
                    e.title("Starboard");
                    e.description(
                        "Starboard is not activated! To do so, use `!starboard enable` command.",
                    );
                    e
                })
            })
            .await?;
        return Ok(());
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.color(0x2F3136);
                e.title("Starboard Config");
                e.field(
                    "Starboard Activate",
                    starboard_config.starboard_activate.unwrap(),
                    true,
                );
                e.field(
                    "Starboard Channel",
                    format!("<#{}>", starboard_config.starboard_channel.unwrap()),
                    true,
                );
                e.field(
                    "Starboard Threshold",
                    starboard_config.starboard_threshold.unwrap(),
                    true,
                );
                e
            })
        })
        .await?;

    Ok(())
}
