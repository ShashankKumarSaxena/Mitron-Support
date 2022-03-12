extern crate serenity;

pub mod cmds;
mod commands;
pub mod events;
pub mod listeners;

use cmds::MODERATION_GROUP;
use dotenv;
use listeners::Handler;
use serenity::{
    framework::{
        standard::{CommandGroup, CommandResult},
        StandardFramework,
    },
    http::Http,
    model::webhook::Webhook,
    prelude::*,
};
use std::collections::HashSet;
use std::env;
use tokio;
use tracing::{error, info, instrument};
use tracing_subscriber;

#[tokio::main]
#[instrument]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file!");
    tracing_subscriber::fmt::init();

    // Fetching data from .env
    let TOKEN: String = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN not set!");
    let APPLICATION_ID: u64 = env::var("APPLICATION_ID")
        .expect("APPLICATION_ID not set!")
        .parse()
        .expect("APPLICATION_ID not a number!");

    let http = Http::new_with_token(&TOKEN);

    // Fetching application's information
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => {
            error!("Could not access application info: {:?}", why);
            panic!("[CORE FAILURE] Shutting down...");
        }
    };

    // Making simple command framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&MODERATION_GROUP);

    info!("Commands loaded!");

    // Connect Database

    // Making bot instance
    let mut bot = Client::builder(&TOKEN)
        .application_id(APPLICATION_ID)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating bot instance.");
    {
        let mut data = bot.data.write().await;
    }

    info!("Core initialisation successfull! Attempting to start...");

    // Starting bot
    if let Err(why) = bot.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
