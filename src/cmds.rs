use crate::commands::moderation::basic::*;
use crate::commands::reaction_roles::reactionroles::*;
use crate::commands::starboard::starboard::*;
use crate::commands::welcomer::autoroles::*;
use crate::commands::welcomer::welcome::*;
use crate::commands::status::botstatus::*;
use serenity::framework::standard::macros::group;

// All commands related with moderation
#[group]
#[commands(kick, ban, unban)]
#[summary = "Moderation Commands"]
pub struct Moderation;

// All utility commands.
#[group]
#[commands(
    welcome,
    welcome_disable,
    welcome_message,
    welcome_image,
    reactionroles,
    autorole,
    autorole_add,
    autorole_remove,
    autorole_list,
    starboard,
    watcher,
    announce,
    announce_simple
)]
#[summary = "Utility Commands"]
pub struct Utility;
