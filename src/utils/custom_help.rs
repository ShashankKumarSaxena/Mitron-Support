use serenity::framework::standard::{Args, CommandGroup, HelpOptions};

use serenity::framework::standard::help_commands::{
    create_customised_help_data, Command, CustomisedHelpData, GroupCommandsPair, Suggestions,
};

use serenity::model::id::UserId;
use std::{collections::HashSet, fmt::Write};

use serenity::{
    builder, client::Context, http::Http, model::channel::Message, model::id::ChannelId,
    utils::Colour, Error,
};

use tracing::warn;

// macro_rules! format_command_name {
//     ($behaviour:expr, $command_name:expr) => {
//         match $behaviour {
//             HelpBehaviour::Strike => format!("~~`{}`~~", $command_name),
//             HelpBehaviour::Nothing => format!("`{}`", $command_name),
//             HelpBehaviour::Hide => continue,
//         }
//     };
// }

macro_rules! warn_about_failed_send {
    ($customised_help:expr, $error:expr) => {
        warn!(
            "Failed to send {:?} because: {:?}",
            $customised_help, $error
        );
    };
}

async fn send_suggestion_embed(
    http: impl AsRef<Http>,
    channel_id: ChannelId,
    help_description: &str,
    suggestions: &Suggestions,
    colour: Colour,
) -> Result<Message, Error> {
    let text = help_description.replace("{}", &suggestions.join("`, `"));

    channel_id
        .send_message(&http, |m| {
            m.embed(|e| {
                e.colour(colour);
                e.description(text);
                e
            });
            m
        })
        .await
}

async fn send_error_embed(
    http: impl AsRef<Http>,
    channel_id: ChannelId,
    input: &str,
    colour: Colour,
) -> Result<Message, Error> {
    channel_id
        .send_message(&http, |m| {
            m.embed(|e| {
                e.colour(colour);
                e.description(input);
                e
            });
            m
        })
        .await
}

fn flatten_group_to_string(
    ctx: &Context,
    group_text: &mut String,
    group: &GroupCommandsPair,
    nest_level: usize,
    help_options: &HelpOptions,
) {
    let repeated_indent_str = help_options.indention_prefix.repeat(nest_level);

    if nest_level > 0 {
        let _ = writeln!(group_text, "{}__**{}**__", repeated_indent_str, group.name,);
    }

    let mut single_group: bool = false;
    if !group.prefixes.is_empty() {
        let _ = writeln!(
            group_text,
            "{}{}: `{}`",
            &repeated_indent_str,
            help_options.group_prefix,
            group.prefixes.join("`, `"),
        );
    };
    let mut sep = ", ";
    if single_group {
        sep = "\n";
    }

    let mut joined_commands = String::from(format!("*{}*\n", group.summary.unwrap()));

    joined_commands.push_str(group
        .command_names
        .join(&format!("{}{}", sep, &repeated_indent_str)).as_str());

    if !group.command_names.is_empty() {
        joined_commands.insert_str(0, &repeated_indent_str);
    }

    let _ = writeln!(group_text, "{}", joined_commands);

    for sub_group in &group.sub_groups {
        if !(sub_group.command_names.is_empty() && sub_group.sub_groups.is_empty()) {
            let mut sub_group_text = String::default();

            flatten_group_to_string(
                ctx,
                &mut sub_group_text,
                &sub_group,
                nest_level + 1,
                &help_options,
            );

            let _ = write!(group_text, "{}", sub_group_text);
        }
    }
}

// Sends the main help embed in the front with `help` command.
async fn send_grouped_commands_embed(
    ctx: &Context,
    http: impl AsRef<Http>,
    help_options: &HelpOptions,
    message: Message,
    channel_id: ChannelId,
    help_description: &str,
    groups: &[GroupCommandsPair],
    colour: Colour,
) -> Result<Message, Error> {

    let mut embed = builder::CreateEmbed::default();

    embed.author(|a| {
        a.name(String::from("Miत्रों Support Help Command"));
        a.icon_url(String::from(
            "https://cdn.discordapp.com/attachments/827217147486797875/952943506836582440/blue.png",
        ));
        a
    });
    // embed.colour(colour);
    if groups.len() != 1 {
        embed.description(format!("{}\n⚠️ **Note:** Remember `[]` = Optional Parameter and `<>` = Required Paramter. Do *not* type these when using commands.", help_description));
    }

    for group in groups {
        let mut embed_text = String::default();

        flatten_group_to_string(ctx, &mut embed_text, group, 0, help_options);

        embed.field(group.name, &embed_text, false);
    }

    channel_id.send_message(&http, |m| m.set_embed(embed)).await;
    Ok(message)
}

// #[cfg(all(feature = "cache", feature = "http"))]
async fn send_single_command_embed(
    http: impl AsRef<Http>,
    help_options: &HelpOptions,
    channel_id: ChannelId,
    command: &Command<'_>,
    colour: Colour,
) -> Result<Message, Error> {
    channel_id
        .send_message(&http, |m| {
            m.embed(|embed| {
                // embed.title(&command.name);
                embed.title(&command.group_name);
                embed.colour(colour);
                embed.author(|a| {
                    a.name(String::from("Miत्रों Support Help Command"));
                    a.icon_url(String::from(
                        "https://cdn.discordapp.com/attachments/827217147486797875/952943506836582440/blue.png",
                    ));
                    a
                });

                let mut descrip = "";
                if let Some(ref desc) = command.description {
                    // embed.description(desc);
                    descrip = desc;
                }

                let mut usage_txt = String::from("");

                if let Some(ref usage) = command.usage {
                    let full_usage_text = if let Some(first_prefix) = command.group_prefixes.get(0) {
                        format!("{} {} {}", first_prefix, command.name, usage)
                    } else {
                        format!("{} {}", command.name, usage)
                    };

                    // embed.title(format!("{} {}", command.name, full_usage_text));
                    usage_txt = full_usage_text;
                }

                embed.description(format!("```\nUsage: {}\n```\n{}", usage_txt, descrip));

                if !command.usage_sample.is_empty() {
                    let full_example_text = if let Some(first_prefix) = command.group_prefixes.get(0) {
                        let format_example = |example| format!(" - `{} {} {}`\n", first_prefix, command.name, example);
                        command.usage_sample.iter().map(format_example).collect::<String>()
                    } else {
                        let format_example = |example| format!(" - `{} {}`\n", command.name, example);
                        command.usage_sample.iter().map(format_example).collect::<String>()
                    };
                    embed.field(&help_options.usage_sample_label, full_example_text, false);
                }

                // embed.field(&help_options.grouped_label, command.group_name, false);

                if !command.aliases.is_empty() {
                    embed.field(&help_options.aliases_label, format!("`{}`", command.aliases.join("`, `")), false);
                }

                if !help_options.available_text.is_empty() && !command.availability.is_empty() {
                    embed.field(&help_options.available_text, &command.availability, true);
                }

                if !command.checks.is_empty() {
                    embed.field(&help_options.checks_label, format!("`{}`", command.checks.join("`, `")), false);
                }

                if !command.sub_commands.is_empty() {
                    embed.field(&help_options.sub_commands_label, format!("`{}`", command.sub_commands.join("`, `")), false);
                }

                embed
            });
            m
        })
        .await
}

pub async fn with_embeds(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> Option<Message> {
    // ) -> Option<Message>
    let formatted_help =
        create_customised_help_data(ctx, msg, &args, groups, &owners, help_options).await;

    let response_result = match formatted_help {
        CustomisedHelpData::SuggestedCommands {
            ref help_description,
            ref suggestions,
        } => {
            send_suggestion_embed(
                &ctx.http,
                msg.channel_id,
                help_description,
                suggestions,
                help_options.embed_error_colour,
            )
            .await
        }
        CustomisedHelpData::NoCommandFound { help_error_message } => {
            send_error_embed(
                &ctx.http,
                msg.channel_id,
                "No such command found. To see the list of commands use `[p]help` command.",
                help_options.embed_error_colour,
            )
            .await
        }
        CustomisedHelpData::GroupedCommands {
            ref help_description,
            ref groups,
        } => {
            send_grouped_commands_embed(
                ctx,
                &ctx.http,
                help_options,
                msg.clone(),
                msg.channel_id,
                "To get help with an individual command, pass its name as an argument to this command. ~~`Strikethrough commands`~~ are unavailable because they require permissions, require a specific role, require certain conditions, or are limited to server messages.",
                groups,
                help_options.embed_success_colour,
            )
            .await
        }
        CustomisedHelpData::SingleCommand { ref command } => {
            send_single_command_embed(
                &ctx.http,
                help_options,
                msg.channel_id,
                command,
                help_options.embed_success_colour,
            )
            .await
        }
        _ => {
            panic!("Something went wrong!");
        }
    };

    match response_result {
        Ok(response) => Some(response),
        Err(why) => {
            warn_about_failed_send!(&formatted_help, why);
            None
        }
    }
}
