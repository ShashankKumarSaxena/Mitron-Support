use crate::utils::typemaps::PgConnectionPool;
use serenity::model::interactions::Interaction;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::interactions::InteractionType;
use serenity::{model::prelude::*, prelude::*};
use sqlx::Row;
use std::i64;

pub async fn interaction_create(ctx: Context, interaction: Interaction) {
    if interaction.kind() == InteractionType::MessageComponent {
        let msg_component = interaction.clone().message_component().unwrap();
        let role_id = msg_component.data.custom_id.parse::<u64>().unwrap();

        let db = ctx
            .data
            .read()
            .await
            .get::<PgConnectionPool>()
            .unwrap()
            .clone();

        let cur = sqlx::query("SELECT guild_id FROM reactionrole WHERE $1 = ANY(roles);")
            .bind(role_id as i64)
            .fetch_one(&db)
            .await
            .unwrap();

        let guild_id = cur.try_get::<i64, _>("guild_id").unwrap();
        let roles = ctx.http.get_guild_roles(guild_id as u64).await.unwrap();

        let role = roles.iter().find(|r| r.id == role_id).unwrap();

        let mut mem = ctx
            .http
            .get_member(guild_id as u64, msg_component.user.id.0)
            .await
            .unwrap();
        let mut has: bool = false;

        for mem_role in mem.roles.iter() {
            if mem_role.0 == role.id.0 {
                has = true;
                break;
            }
        }

        if has {
            mem.remove_role(&ctx.http, role.id.0).await.unwrap();

            msg_component
                .create_interaction_response(&ctx.http, |i| {
                    i.kind(InteractionResponseType::DeferredChannelMessageWithSource);
                    i.interaction_response_data(|d| {
                        d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                        d.content(
                            format!("You have been removed from the role **{}**.", role.name)
                                .as_str(),
                        );
                        d
                    });
                    i
                })
                .await;
        } else {
            mem.add_role(&ctx.http, role.id.0).await.unwrap();

            msg_component
                .create_interaction_response(&ctx.http, |i| {
                    i.kind(InteractionResponseType::DeferredChannelMessageWithSource);
                    i.interaction_response_data(|d| {
                        d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                        d.content(format!("You have been give **{}** role.", role.name).as_str());
                        d
                    });
                    i
                })
                .await;
        }
    }
}
