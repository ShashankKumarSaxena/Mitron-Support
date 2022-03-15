use serenity::{
    builder::{CreateActionRow, CreateButton},
    framework::standard::{macros::command, Args, CommandResult},
    model::interactions::message_component::{ActionRow, ButtonStyle},
    model::prelude::*,
    prelude::*,
};
use std::str::FromStr;

use crate::utils::typemaps::PgConnectionPool;

struct ReactionRoles {
    role_id: u64,
    role_title: String,
}

impl ReactionRoles {
    fn make_button(&self) -> CreateButton {
        let mut b = CreateButton::default();
        b.custom_id(self.role_id);
        b.emoji(ReactionType::from_str(self.role_title.clone().as_str()).unwrap());
        b.style(ButtonStyle::Unknown);
        b
    }
}

async fn send_timeout_msg(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Reaction Roles");
                e.description("⏰ Time's up. To start the setup again, run the command again.");
                e
            })
        })
        .await?;
    return Ok(());
}

#[command]
#[description("Creates a reaction role message.")]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn reactionroles(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Reaction Roles");
                e.description("Mention the channel in which you want to setup reaction roles.");
                e
            })
        })
        .await?;

    let channel = match &msg
        .author
        .await_reply(&ctx)
        .timeout(std::time::Duration::from_secs(30))
        .await
    {
        Some(content) => {
            if content.content.is_empty() {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Reaction Roles");
                            e.description("You need to mention a channel.");
                            e
                        })
                    })
                    .await?;
                return Ok(());
            } else {
                content.content.parse::<ChannelId>().unwrap()
            }
        }
        None => {
            return send_timeout_msg(&ctx, &msg).await;
        }
    };

    msg.channel_id.send_message(&ctx.http, |m| {
       m.embed(|e| {
           e.title("Reaction Roles");
           e.description("Enter the number of roles you want to add.\n*Make sure the number lie between 1 and 25!*");
           e
       })
   }).await?;

    let role_count = match msg
        .author
        .await_reply(&ctx)
        .timeout(std::time::Duration::from_secs(30))
        .await
    {
        Some(content) => {
            if content.content.is_empty() {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Reaction Roles");
                            e.description("You need to enter a number.");
                            e
                        })
                    })
                    .await?;
                return Ok(());
            } else {
                content.content.parse::<u8>().unwrap()
            }
        }
        None => {
            return send_timeout_msg(&ctx, &msg).await;
        }
    };

    if role_count > 25 {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Reaction Roles");
                    e.description("Invalid number of roles provided!");
                    e
                })
            })
            .await?;
        return Ok(());
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Reaction Roles");
            e.description("Mention the roles which you want to add.\n*Make sure to mention the roles as provided above!*");
            e
        })
    }).await?;

    let roles = match msg
        .author
        .await_reply(&ctx)
        .timeout(std::time::Duration::from_secs(30))
        .await
    {
        Some(content) => {
            let mentioned_roles = content.mention_roles.clone();
            if mentioned_roles.len() == 0 {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Reaction Roles");
                            e.description("You need to mention roles.");
                            e
                        })
                    })
                    .await?;
                return Ok(());
            } else if mentioned_roles.len() < role_count.into() {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Reaction Roles");
                            e.description("You need to mention the roles as provided above.");
                            e
                        })
                    })
                    .await?;
                return Ok(());
            } else {
                mentioned_roles
            }
        }
        None => {
            return send_timeout_msg(&ctx, &msg).await;
        }
    };

    let client_mem = ctx
        .http
        .get_member(
            msg.guild_id.unwrap().0,
            ctx.http.get_current_user().await.unwrap().id.0,
        )
        .await
        .unwrap();
    for role in roles.clone() {
        if role > client_mem.roles[0] {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Reaction Roles");
                        e.description("A role is above my top role. Please run the command again.");
                        e
                    })
                })
                .await?;
            return Ok(());
        }
    }

    let mut role_titles: Vec<String> = vec![];
    let mut role_descriptions: Vec<String> = vec![];

    for (_idx, role) in roles.clone().iter().enumerate() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Reaction Roles");
                    e.description(format!("Enter the emoji for role <@&{}>", role.0));
                    e
                })
            })
            .await?;

        let role_title = match msg.author.await_reply(&ctx).await {
            Some(content) => {
                if content.content.is_empty() {
                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.title("Reaction Roles");
                                e.description("You need to enter a emoji.");
                                e
                            })
                        })
                        .await?;
                    return Ok(());
                } else {
                    content.content.parse::<String>().unwrap()
                }
            }
            None => {
                return send_timeout_msg(&ctx, &msg).await;
            }
        };

        role_titles.push(role_title);
    }

    for (_idx, role) in roles.clone().iter().enumerate() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Reaction Roles");
                    e.description(format!("Enter the description for role <@&{}>", role.0));
                    e
                })
            })
            .await?;

        let role_description = match msg.author.await_reply(&ctx).await {
            Some(content) => {
                if content.content.is_empty() {
                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.title("Reaction Roles");
                                e.description("You need to enter a description.");
                                e
                            })
                        })
                        .await?;
                    return Ok(());
                } else {
                    content.content.parse::<String>().unwrap()
                }
            }
            None => {
                return send_timeout_msg(&ctx, &msg).await;
            }
        };

        role_descriptions.push(role_description);
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Reaction Roles");
                e.description("Enter the title of embed.\n*Enter **skip** to skip this step!*");
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
                            e.title("Reaction Roles");
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
        None => {
            return send_timeout_msg(&ctx, &msg).await;
        }
    };

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Reaction Roles");
                e.description(
                    "Enter the description of embed.\n*Enter **skip** to skip this step!*",
                );
                e
            })
        })
        .await?;

    let embed_description = match msg.author.await_reply(&ctx).await {
        Some(content) => {
            if content.content.is_empty() {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Reaction Roles");
                            e.description("You need to enter a description.");
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
        None => {
            return send_timeout_msg(&ctx, &msg).await;
        }
    };

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Reaction Roles");
                e.description("Enter the image URL of embed.\n*Enter **skip** to skip this step!*");
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
                            e.title("Reaction Roles");
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
        None => {
            return send_timeout_msg(&ctx, &msg).await;
        }
    };

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Reaction Roles");
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
                            e.title("Reaction Roles");
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
        None => {
            return send_timeout_msg(&ctx, &msg).await;
        }
    };

    // Add the stuff in database
    let mut role_ids: Vec<i64> = vec![];
    for role in roles.iter() {
        role_ids.push(role.0 as i64);
    }

    // Create reaction embed/prompt
    let mut embed_desc: String = String::new();

    let rr_msg = channel
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                if !embed_title.is_empty() {
                    e.title(embed_title);
                };

                if !embed_description.is_empty() {
                    embed_desc += format!("{}\n\n", embed_description).as_str();
                };

                if !embed_image.is_empty() {
                    e.image(embed_image);
                }

                if !embed_thumbnail.is_empty() {
                    e.thumbnail(embed_thumbnail);
                }

                for (idx, title) in role_titles.iter().enumerate() {
                    embed_desc +=
                        format!("{} - {}\n", title, role_descriptions[idx].clone()).as_str();
                }

                e.description(embed_desc);

                e
            });

            // Create Interactions in the message
            let mut components: Vec<ReactionRoles> = vec![];

            for (idx, role) in roles.iter().enumerate() {
                let r = ReactionRoles {
                    role_id: role.0,
                    role_title: role_titles[idx].clone(),
                };

                components.push(r);
            }

            // let mut action_rows: Vec<CreateActionRow> = vec![];
            m.components(|c| {
                let mut action_row: CreateActionRow = CreateActionRow::default();
                for (_idx, rr) in components.iter().enumerate() {
                    action_row.add_button(rr.make_button());
                }
                c.add_action_row(action_row);
                c
            });
            m
        })
        .await?;

    let db = ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    sqlx::query("INSERT INTO reactionrole (guild_id, roles, message_id, titles, descriptions) VALUES ($1, $2, $3, $4, $5);")
        .bind(msg.guild_id.unwrap().0 as i64)
        .bind(role_ids.clone())
        .bind(rr_msg.id.0 as i64)
        .bind(role_titles.clone())
        .bind(role_descriptions.clone())
        .execute(&db)
        .await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Reaction Roles");
                e.description("✅ Successfully setup reaction roles!");
                e
            })
        })
        .await?;

    Ok(())
}
