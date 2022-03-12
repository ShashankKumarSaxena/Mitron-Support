use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[description("Kicks a user from server.")]
#[usage("<member> [reason]")]
#[example("@user#1234 Abusing.")]
#[only_in(guilds)]
#[required_permissions("KICK_MEMBERS")]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    
    Ok(())
}