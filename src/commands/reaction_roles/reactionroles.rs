use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    model::user::User,
    prelude::*,
};

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
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Reaction Roles");
                        e.description(
                            "⏰ Time's up. To start the setup again, run the command again.",
                        );
                        e
                    })
                })
                .await?;
            return Ok(());
        }
    };

    msg.channel_id.say(&ctx.http, format!("{}", channel.0)).await?;

    Ok(())
}
