use crate::commands::moderation::basic::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(kick, ban, unban)]
#[summary = "Moderation Commands"]
pub struct Moderation;
