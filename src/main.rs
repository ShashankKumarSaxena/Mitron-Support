use dotenv;
use serenity::{
    framework::{
        standard::{CommandGroup, CommandResult},
        StandardFramework,
    },
    http::Http,
    model::webhook::Webhook,
    prelude::*,
};
use tokio;
use tracing_subscriber;

#[tokio::main]
async fn main() -> CommandResult {
    dotenv::dotenv().expect("Failed to load .env file!");

    Ok(())
}
