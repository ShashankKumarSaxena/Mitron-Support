use crate::utils::typemaps::PgConnectionPool;
use serenity::{builder::CreateEmbed, model::prelude::*, prelude::*};
use sqlx::Executor;

pub async fn reaction_add(ctx: &Context, add_reaction: Reaction) {
    match add_reaction.guild_id {
        Some(_) => {}
        None => return,
    };

    if add_reaction.emoji.as_data() != "⭐" {
        return;
    }

    let pool = &ctx
        .data
        .read()
        .await
        .get::<PgConnectionPool>()
        .unwrap()
        .clone();

    let guild_data = sqlx::query!(
        "SELECT * FROM guildconfig WHERE id = $1",
        add_reaction.guild_id.unwrap().0 as i64
    )
    .fetch_one(pool)
    .await
    .unwrap();

    if guild_data.starboard_activate.unwrap() == false {
        return;
    }

    let reactions = add_reaction.message(&ctx.http).await.unwrap().reactions;
    let stars = match reactions
        .into_iter()
        .find(|x| x.reaction_type.as_data() == "⭐")
    {
        Some(r) => r.count,
        None => 0,
    };

    if stars < guild_data.starboard_threshold.unwrap() as u64 {
        return;
    }

    let flag = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM starboard_message WHERE message_id = $1);",
        add_reaction.message_id.0 as i64
    )
    .fetch_one(pool)
    .await
    .unwrap();

    if flag.exists.unwrap() {
        return;
    }

    sqlx::query!("INSERT INTO starboard_message (stars_count, message_id, guild_id, author_id, channel_id) VALUES ($1, $2, $3, $4, $5)", stars as i64, add_reaction.message_id.0 as i64, add_reaction.guild_id.unwrap().0 as i64, add_reaction.user_id.unwrap().0 as i64, add_reaction.channel_id.0 as i64)
                    .execute(pool)
                    .await
                    .unwrap();

    let mut starboard_msg_data = match sqlx::query!(
        "SELECT * FROM starboard_message WHERE message_id = $1",
        add_reaction.message_id.0 as i64
    )
    .fetch_one(pool)
    .await
    {
        Ok(data) => data,
        Err(_) => {
            return;
        }
    };

    if guild_data.starboard_threshold <= starboard_msg_data.stars_count {
        // Post the message in the starboard channel
        let starboard_channel = ctx
            .http
            .get_channel(guild_data.starboard_channel.unwrap() as u64)
            .await
            .unwrap();

        let starboard_reaction_message = add_reaction.message(&ctx.http).await.unwrap();
        let starboard_author = ctx
            .http
            .get_member(
                add_reaction.guild_id.unwrap().0,
                starboard_msg_data.author_id.unwrap() as u64,
            )
            .await
            .unwrap();

        starboard_channel
            .id()
            .send_message(&ctx.http, |m| {
                m.content(format!(
                    "💫 **{}** <#{}> ID: {}",
                    starboard_msg_data.stars_count.unwrap(),
                    starboard_msg_data.channel_id.unwrap(),
                    starboard_msg_data.message_id.unwrap()
                ));
                m.embed(|e| {
                    e.author(|a| {
                        a.name(&starboard_author.user.name);
                        a.icon_url(&starboard_author.user.face());
                        a
                    });

                    e.description(&starboard_reaction_message.content);

                    if starboard_reaction_message.attachments.len() > 0 {
                        e.image(starboard_reaction_message.attachments[0].url.clone());
                    }

                    if starboard_reaction_message.embeds.len() > 0 {
                        let emb = CreateEmbed::from(starboard_reaction_message.embeds[0].clone());
                        e.0.clone_from(&emb.0);
                    }

                    e.field(
                        "Original",
                        format!(
                            "[Jump!](https://discordapp.com/channels/{}/{}/{})",
                            guild_data.id,
                            starboard_reaction_message.channel_id.0,
                            starboard_reaction_message.id.0
                        ),
                        false,
                    );

                    e
                });

                m
            })
            .await;
    }
}
