use dotenv::dotenv;
use std::env;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::application::ResolvedOption;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::model::application::CommandOptionType;
use serenity::builder::{CreateCommand, CreateCommandOption};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}