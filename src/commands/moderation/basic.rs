use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    model::user::User,
    prelude::*,
};

use tracing::warn;

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
                        e.color(0x2F3136);
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
                e.color(0x2F3136);
                if failed_txt.is_empty() {
                    e.description("✅ Successfully kicked members.");
                } else {
                    e.description(format!(
                        "✅ Successfully kicked members.\n\n**Warnings:**\n{}\n*Please check my permissions once!*",
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
                        e.color(0x2F3136);
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
                        failed_txt.push_str(format!("⚠️ Failed to ban: {}\n", member.name).as_str());
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
                e.color(0x2F3136);
                if failed_txt.is_empty() {
                    e.description("✅ Successfully banned members.");
                } else {
                    e.description(format!(
                        "✅ Successfully banned members.\n\n**Warnings:**\n{}\n*Please check my permissions once!*",
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
#[description("Unban a banned user from server.")]
#[usage("<user_id>")]
#[example("766553763569336340")]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
async fn unban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        return {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(0x2F3136);
                        e.description("❌ You need to mention a user to unban.");
                        e
                    })
                })
                .await?;
            Ok(())
        };
    };

    let user_id = args.single::<u64>().unwrap();

    let mut user: User = match ctx.http.get_user(user_id).await {
        Err(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(0x2F3136);
                        e.description("❌ User not found.");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
        Ok(user) => user,
    };

    match msg.guild_id {
        Some(guild_id) => match guild_id.unban(&ctx.http, user.id).await {
            Err(_) => {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.color(0x2F3136);
                            e.description("❌ Failed to unban user.");
                            e
                        })
                    })
                    .await?;
                return Ok(());
            }
            Ok(_) => {
                let b_user = &user;
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.color(0x2F3136);
                            e.description(
                                format!("✅ Successfully unbanned {}.", b_user.name).as_str(),
                            );
                            e
                        })
                    })
                    .await?;
                return Ok(());
            }
        },
        None => {}
    }
    Ok(())
}

#[command("announce-simple")]
#[description("Announce something in a channel with bot in simple text.")]
#[usage("<channel> <text>")]
#[example("#announcements Today is holiday!")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn announce_simple(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    match args.single::<ChannelId>() {
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Please mention a channel to send announcement in!",
                )
                .await?;

            return Ok(());
        }
        Ok(ch) => {
            let text = args.rest();
            if text.is_empty() {
                msg.channel_id
                    .say(&ctx.http, "Provide a text to announce!")
                    .await?;
                return Ok(());
            }
            // match ch.say(&ctx.http, text).await {
            match ch
                .send_message(&ctx.http, |m| {
                    m.content(text);
                    m
                })
                .await
            {
                Ok(_) => {
                    msg.channel_id
                        .say(&ctx.http, "Successfully sent announcement!")
                        .await?;
                }
                Err(_) => {
                    msg.channel_id
                        .say(
                            &ctx.http,
                            "Can't send message to that channel! Please check my permissions.",
                        )
                        .await?;
                }
            }
        }
    };

    Ok(())
}

#[command]
#[description("Announce something in a channel from bot in a embed.")]
#[usage("<channel> <text>")]
#[example("#announcements Today is holiday!")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn announce(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    match args.single::<ChannelId>() {
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Please mention a channel to send announcement in!",
                )
                .await?;

            return Ok(());
        }
        Ok(ch) => {
            let text = args.rest();
            if text.is_empty() {
                msg.channel_id
                    .say(&ctx.http, "Provide a text to announce!")
                    .await?;
                return Ok(());
            }

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(0x2F3136);
                        e.description(
                            "Enter the title of embed.\n*Enter **skip** to skip this step!*",
                        );
                        e
                    })
                })
                .await?;

            let embed_title = match msg.author.await_reply(&ctx).await {
                Some(content) => {
                    if content.content.is_empty() {
                        msg.channel_id
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| {
                                    e.color(0x2F3136);
                                    e.description("You need to enter a title.");
                                    e
                                })
                            })
                            .await?;
                        return Ok(());
                    } else if content.content.to_lowercase() == "skip" {
                        "".to_string()
                    } else {
                        content.content.parse::<String>().unwrap()
                    }
                }
                None => String::from(""),
            };

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(0x2F3136);
                        e.description(
                            "Enter the image URL of embed.\n*Enter **skip** to skip this step!*",
                        );
                        e
                    })
                })
                .await?;

            let embed_image = match msg.author.await_reply(&ctx).await {
                Some(content) => {
                    if content.content.is_empty() {
                        msg.channel_id
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| {
                                    e.color(0x2F3136);
                                    e.description("You need to enter a image URL.");
                                    e
                                })
                            })
                            .await?;
                        return Ok(());
                    } else if content.content.to_lowercase() == "skip" {
                        "".to_string()
                    } else {
                        content.content.parse::<String>().unwrap()
                    }
                }
                None => String::from(""),
            };

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.color(0x2F3136);
                        e.description(
                        "Enter the thumbnail URL of embed.\n*Enter **skip** to skip this step!*",
                    );
                        e
                    })
                })
                .await?;

            let embed_thumbnail = match msg.author.await_reply(&ctx).await {
                Some(content) => {
                    if content.content.is_empty() {
                        msg.channel_id
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| {
                                    e.color(0x2F3136);
                                    e.description("You need to enter a URL.");
                                    e
                                })
                            })
                            .await?;
                        return Ok(());
                    } else if content.content.to_lowercase() == "skip" {
                        "".to_string()
                    } else {
                        content.content.parse::<String>().unwrap()
                    }
                }
                None => String::from(""),
            };

            // match ch.say(&ctx.http, text).await {
            match ch
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(text);
                        e.color(0x2F3136);

                        if !embed_image.is_empty() {
                            e.image(embed_image);
                        }

                        if !embed_thumbnail.is_empty() {
                            e.thumbnail(embed_thumbnail);
                        }

                        if !embed_title.is_empty() {
                            e.title(embed_title);
                        }

                        e
                    });
                    m
                })
                .await
            {
                Ok(_) => {
                    msg.channel_id
                        .say(&ctx.http, "Successfully sent announcement!")
                        .await?;
                }
                Err(_) => {
                    msg.channel_id
                        .say(
                            &ctx.http,
                            "Can't send message to that channel! Please check my permissions.",
                        )
                        .await?;
                }
            }
        }
    };

    Ok(())
}
