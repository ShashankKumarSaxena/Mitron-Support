use crate::utils::typemaps::PgConnectionPool;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use sqlx::Row;
use std::i64;
use tracing::warn;

#[command("autorole")]
#[description("Setup autoroles.")]
#[usage("<role>")]
#[example("@Member")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn autorole(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Autorole Setup");
            e.description("`autorole-add`: Add a role to give to newly joined members.\n`autorole-remove`: Remove a role from autoroles.\n`autorole-list`: List all autoroles configured.");
            e
        })
    }).await.unwrap();
    Ok(())
}

#[command("autorole-add")]
#[description("Add a role to give to members who join the server.")]
#[usage("<role>")]
#[example("@Member")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn autorole_add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let role = match args.single::<RoleId>() {
        Err(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("⚠️ Role not provided!");
                        e.description("Please mention a role to setup autoroles.");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
        Ok(r) => r,
    };

    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let v = sqlx::query("SELECT id FROM guildconfig WHERE $1 = ANY(autoroles) AND id = $2")
        .bind(role.0 as i64)
        .bind(msg.guild_id.unwrap().0 as i64)
        .fetch_one(&db)
        .await?;

    match v.try_get::<i64, _>("id") {
        Ok(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("⚠️ Role already addeds!");
                        e.description("The role you specified is already in the autoroles list.");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
        Err(_) => {}
    }

    sqlx::query("UPDATE guildconfig SET autoroles = ARRAY_APPEND(autoroles, $1) WHERE id = $2;")
        .bind(role.0 as i64)
        .bind(msg.guild_id.unwrap().0 as i64)
        .execute(&db)
        .await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Autorole Added");
                e.description(format!(
                    "Successfully added <@&{}> role to autoroles!",
                    role.0
                ));
                e
            })
        })
        .await?;

    Ok(())
}

#[command("autorole-remove")]
#[description("Removes a role from autorole.")]
#[usage("<role>")]
#[example("@Member")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn autorole_remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let role = match args.single::<RoleId>() {
        Err(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("⚠️ Role not provided!");
                        e.description("Please mention a role to remove from autoroles.");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
        Ok(r) => r,
    };

    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let v = sqlx::query("SELECT id FROM guildconfig WHERE $1 = ANY(autoroles) AND id = $2")
        .bind(role.0 as i64)
        .bind(msg.guild_id.unwrap().0 as i64)
        .fetch_one(&db)
        .await?;

    match v.try_get::<i64, _>("id") {
        Ok(_) => {}
        Err(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("⚠️ Role not found!");
                        e.description("The role you specified is not in the autoroles list.");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
    }

    sqlx::query("UPDATE guildconfig SET autoroles = ARRAY_REMOVE(autoroles, $1) WHERE id = $2;")
        .bind(role.0 as i64)
        .bind(msg.guild_id.unwrap().0 as i64)
        .execute(&db)
        .await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Autorole Removed");
                e.description(format!(
                    "Successfully removed <@&{}> role from autoroles!",
                    role.0
                ));
                e
            })
        })
        .await?;

    Ok(())
}

#[command("autorole-list")]
#[description("List roles present in autoroles.")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn autorole_list(ctx: &Context, msg: &Message) -> CommandResult {
    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let v = sqlx::query("SELECT autoroles FROM guildconfig WHERE id = $1")
        .bind(msg.guild_id.unwrap().0 as i64)
        .fetch_one(&db)
        .await?;

    let mut roles = match v.try_get::<Vec<i64>, _>("autoroles") {
        Ok(r) => r,
        Err(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("⚠️ No autoroles found!");
                        e.description("There are no autoroles configured for this server.");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
    };

    if roles.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("⚠️ No autoroles found!");
                    e.description("There are no autoroles configured for this server.");
                    e
                })
            })
            .await?;
        return Ok(());
    }

    let mut s = String::new();
    for r in roles.iter() {
        s.push_str(&format!("<@&{}> ", r));
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Autoroles");
                e.description(s);
                e
            })
        })
        .await?;

    Ok(())
}
