mod commands;

use dotenv::dotenv;
use std::env;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse,CreateInteractionResponseMessage,CreateCommand, CreateCommandOption};
use serenity::model::application::{Interaction,CommandOptionType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}"); 
            let content = match command.data.name.as_str() {
                "ping" => {
                    println!("Ping is working!");
                    commands::ping::run_ping()
                },
                "search" => {
                    println!("Search is working!");
                    commands::search::run_search()
                },
                _ => {"Error".to_string()}
            };
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
        let _ = guild_id.set_commands(&ctx.http, vec![
                CreateCommand::new("ping")
                    .description("/ping : command to test if the bot is active"),
                CreateCommand::new("search")
                    .description("/search : command looking for a movie, series, ...")
                    .add_option(CreateCommandOption::new(CommandOptionType::String, "platform", "which stream platform to choose")
                        .required(true)
                        .add_string_choice(
                            "astream",
                            "astream"
                        )
                        .add_string_choice(
                            "bstream",
                            "bstream"
                        )
                        .add_string_choice(
                            "cstream",
                            "cstream"
                        )
                        .add_string_choice(
                            "dstream",
                            "dstream"
                        )
                    )
                    .add_option(CreateCommandOption::new(CommandOptionType::String, "name", "name of movie, series, ...")
                        .required(true)
                    )
        ]).await;
    }
}

#[tokio::main]
async fn main() {
    //loads environment variables, configures events and starts the bot
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