use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::help, Args, CommandGroup, CommandResult, HelpOptions},
    model::id::UserId,
    model::prelude::*,
};
use std::collections::HashSet;

#[help]
#[command_not_found_text = "Could not find command: `{}`."]
#[indention_prefix = ""]
#[lacking_permissions = "Nothing"]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ =
        crate::utils::custom_help::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
