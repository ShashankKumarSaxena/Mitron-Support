use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use tracing::{error, info, warn};

#[command]
#[description("Kicks a user from server.")]
#[usage("<member> [reason]")]
#[example("@user#1234 Abusing.")]
#[only_in(guilds)]
#[required_permissions("KICK_MEMBERS")]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let members = &msg.mentions;
    let reason: String;

    if members.len() == 0 {
        return {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description("❌ You need to mention a member to kick.");
                        e
                    })
                })
                .await?;
            Ok(())
        };
    }

    if args.single::<String>().unwrap().is_empty() {
        reason = format!("By: {} [ID: {}]", msg.author.name, msg.author.id);
    } else {
        reason = args.rest().to_string();
    }

    let mut failed_txt: String = String::from("");

    for member in members {
        if member.id == msg.author.id {
            continue;
        }

        match msg.guild_id {
            Some(guild_id) => {
                match guild_id
                    .kick_with_reason(&ctx.http, member.id, reason.as_str())
                    .await
                {
                    Ok(_) => {}
                    Err(_) => {
                        failed_txt
                            .push_str(format!("⚠️ Failed to kick: {}\n", member.name).as_str());
                    }
                }
            }
            None => {
                warn!("[CORE] Guild ID is None!");
            }
        }
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                if failed_txt.is_empty() {
                    e.description("✅ Successfully kicked members.");
                } else {
                    e.description(format!(
                        "✅ Successfully kicked members.\n\n**Failed to kick following:**\n{}\n*Please check my permissions once!*",
                        failed_txt
                    ).as_str());
                }
                e
            })
        })
        .await?;

    Ok(())
}

#[command]
#[description("Bans a user from server.")]
#[usage("<member> [reason]")]
#[example("@user#1234 Abusing.")]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let members = &msg.mentions;
    let reason: String;

    if members.len() == 0 {
        return {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description("❌ You need to mention a member to ban.");
                        e
                    })
                })
                .await?;
            Ok(())
        };
    }

    if args.single::<String>().unwrap().is_empty() {
        reason = format!("By: {} [ID: {}]", msg.author.name, msg.author.id);
    } else {
        reason = args.rest().to_string();
    }

    let mut failed_txt: String = String::from("");

    for member in members {
        if member.id == msg.author.id {
            continue;
        }

        match msg.guild_id {
            Some(guild_id) => {
                match guild_id
                    .ban_with_reason(&ctx.http, member.id, 1, reason.as_str())
                    .await
                {
                    Ok(_) => {}
                    Err(_) => {
                        failed_txt
                            .push_str(format!("⚠️ Failed to ban: {}\n", member.name).as_str());
                    }
                }
            }
            None => {
                warn!("[CORE] Guild ID is None!");
            }
        }
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                if failed_txt.is_empty() {
                    e.description("✅ Successfully banned members.");
                } else {
                    e.description(format!(
                        "✅ Successfully banned members.\n\n**Failed to ban following:**\n{}\n*Please check my permissions once!*",
                        failed_txt
                    ).as_str());
                }
                e
            })
        })
        .await?;

    Ok(())
}