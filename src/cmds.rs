use crate::commands::moderation::basic::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(kick)]
#[summary = "Moderation Commands"]
pub struct Moderation;
