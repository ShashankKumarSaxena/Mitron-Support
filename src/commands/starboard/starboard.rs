use crate::utils::typemaps::PgConnectionPool;
use chrono::NaiveDateTime;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use sqlx::PgPool;
use sqlx::Row;
use std::i64;

// This file is gonna change a lot in future. *probably but yeah it will*
//                                     - ShashankKumarSaxena

#[command("starboard")]
#[required_permissions("MANAGE_GUILD")]
#[sub_commands(enable, disable, threshold)]
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
async fn enable(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().0;

    let pool = &ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    sqlx::query!(
        "UPDATE guildconfig SET starboard_activate = TRUE WHERE id = $1",
        guild_id as i64
    )
    .execute(pool)
    .await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Starboard");
                e.description("Starboard successfully enabled for this server!");
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
        "UPDATE guildconfig SET starboard_activate = FALSE WHERE id = $1",
        guild_id as i64
    )
    .execute(pool)
    .await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
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
