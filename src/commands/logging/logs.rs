use crate::utility::typemaps::PgConnectionPool;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::Row;
use std::collections::HashMap;
use titlecase::titlecase;

#[command]
#[description("See logs configured for this server.")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn logs(ctx: &Context, msg: &Message) -> CommandResult {
    let logs_vec: Vec<&str> = vec!["message_log", "server_log", "member_log", "join_log", "voice_log", "mod_log"];

    let db = ctx.data.read().await.get::<PgConnectionPool>().unwrap().clone();
    let mut cursor = None;

    match msg.guild_id {
        Some(guild_id) => {
            cursor = Some(
                sqlx::query("SELECT * FROM guildconfig WHERE id = $1")
                    .bind(i64::from(guild_id))
                    .fetch_one(&db)
                    .await
                    .unwrap(),
            );
        }

        None => {
            println!("[COMMAND ERROR] Can't get guild ID")
        }
    }

    match cursor {
        Some(cur) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|emb| {
                        emb.title("Logs Configuration");
                        for log_type in logs_vec.iter() {
                            let channel_value;

                            match cur.try_get::<i64, _>(format!("{}_channel", log_type).as_str()) {
                                Ok(val) => channel_value = format!("<#{}>", val),
                                Err(_) => {
                                    channel_value = "Not set yet!".to_string();
                                }
                            }

                            let is_enabled;

                            match cur.try_get::<bool, _>(format!("{}_enabled", log_type).as_str()) {
                                Ok(val) => {
                                    if val == true {
                                        // is_enabled = "Yes";
                                        is_enabled = "✅";
                                    } else {
                                        // is_enabled = "No";
                                        is_enabled = "❌";
                                    }
                                }

                                Err(_) => {
                                    // is_enabled = "No";
                                    is_enabled = "❌";
                                }
                            }

                            emb.field(
                                format!("{}", titlecase(&log_type.replace("_", " "))),
                                format!("**Channel**: {}\n**Enabled**: {}", channel_value, is_enabled),
                                true,
                            );
                        }
                        // emb.colour(0xFFFFFF);
                        emb
                    })
                })
                .await?;
        }
        None => {}
    }

    Ok(())
}

#[command("log-toggle")]
#[description("Toggle a log on/off.")]
#[usage("<log_type>")]
#[example("messagelog")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn log_toggle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let log_types: Vec<&str> = vec!["messagelog", "serverlog", "memberlog", "modlog", "joinlog", "voicelog"];

    if args.is_empty() {
        let mut response: String = "Please select any of the logs from the following to toggle:\n".to_owned();
        response += &log_types.join(", ");

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|emb| {
                emb.title("Log Toggle");
                emb.description(response);
                emb
            })
        }).await?;

        return Ok(());
    }

    let log_choice: String = args.single::<String>().unwrap();

    if !log_types.contains(&log_choice.as_str()) {
        msg.channel_id.say(&ctx.http, "Invalid log type passed!").await?;
        return Ok(());
    }

    let log_toggles: HashMap<&str, &str> = HashMap::from([
        ("messagelog", "message_log_enabled"),
        ("serverlog", "server_log_enabled"),
        ("memberlog", "member_log_enabled"),
        ("modlog", "mod_log_enabled"),
        ("joinlog", "join_log_enabled"),
        ("voicelog", "voice_log_enabled"),
    ]);
    let db = ctx.data.read().await.get::<PgConnectionPool>().unwrap().clone();

    let mut cursor = None;

    match msg.guild_id {
        Some(guild_id) => {
            cursor = Some(
                sqlx::query("SELECT * FROM guild_config WHERE id = $1")
                    .bind(i64::from(guild_id))
                    .fetch_one(&db)
                    .await
                    .unwrap(),
            );
        }

        None => {
            println!("[COMMAND ERROR] Can't get guild ID")
        }
    }

    let mut toggle_bool: bool = true;

    match cursor {
        Some(cur) => match cur.try_get::<bool, _>(log_toggles.get(log_choice.as_str()).unwrap()) {
            Ok(val) => {
                if val == true {
                    toggle_bool = false;
                } else {
                    toggle_bool = true;
                }
            }
            Err(_) => {}
        },
        None => {}
    }

    match msg.guild_id {
        Some(guild_id) => {
            sqlx::query(format!("UPDATE guild_config SET {} = $1 WHERE id = $2", log_toggles.get(log_choice.as_str()).unwrap()).as_str())
                .bind(toggle_bool)
                .bind(i64::from(guild_id))
                .execute(&db)
                .await?;
        }
        None => {
            println!("[COMMAND ERROR] Can't get guild ID")
        }
    }

    if toggle_bool {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|emb| {
                emb.title("Log Toggle");
                emb.description(format!("✅ Successfully enabled {}!", log_choice));
                emb
            })
        }).await?;
    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|emb| {
                emb.title("Log Toggle");
                emb.description(format!("✅ Successfully disabled {}!", log_choice));
                emb
            })
        }).await?;
    }

    Ok(())
}

#[command]
#[description("Set a channel to log stuff in.")]
#[usage("<log_type> <channel>")]
#[example("messagelog #message-logs")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn log_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let log_types: Vec<&str> = vec!["messagelog", "serverlog", "memberlog", "modlog", "joinlog", "voicelog"];

    if args.is_empty() {
        let mut response: String = "Please select any of the logs from the following to set the channel for:\n".to_owned();
        response += &log_types.join(", ");

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|emb| {
                emb.title("Log Toggle");
                emb.description(response);
                emb
            })
        }).await?;

        return Ok(());
    }

    let log_choice = args.single::<String>().unwrap();

    if !log_types.contains(&log_choice.as_str()) {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|emb| {
                emb.description("Invalid log type passed!");
                emb
            })
        }).await?;
        return Ok(());
    }

    let log_toggles: HashMap<&str, &str> = HashMap::from([
        ("messagelog", "message_log_channel"),
        ("serverlog", "server_log_channel"),
        ("memberlog", "member_log_channel"),
        ("modlog", "mod_log_channel"),
        ("joinlog", "join_log_channel"),
        ("voicelog", "voice_log_channel"),
    ]);
    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    // sqlx::query(format!("UPDATE guild_config SET {} = $1 WHERE id = $2", log_toggles.get(log_choice.as_str()).unwrap()), )

    Ok(())
}
