extern crate serenity;

pub mod cmds;
mod commands;
pub mod db;
pub mod events;
pub mod listeners;
pub mod utils;

use crate::utils::typemaps::PgConnectionPool;
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
    let _pool = db::get_pool(
        env::var("DATABASE_URL")
            .expect("DATABASE_URL not set!")
            .as_str(),
    )
    .await
    .unwrap();
    match sqlx::migrate!("./migrations").run(&_pool).await {
        Ok(_) => {
            info!("[DATABASE] Database migrations created!");
        }
        Err(why) => {
            error!("[DATABASE] Database migrations failed: {:?}", why);
            panic!("[CORE FAILURE] Shutting down...");
        }
    };

    info!("[DATABASE] Connected to Database!");

    // Making bot instance
    let mut bot = Client::builder(&TOKEN)
        .application_id(APPLICATION_ID)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating bot instance.");
    {
        let mut data = bot.data.write().await;

        data.insert::<PgConnectionPool>(_pool.clone());
    }

    info!("Core initialisation successfull! Attempting to start...");

    // Starting bot
    if let Err(why) = bot.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
