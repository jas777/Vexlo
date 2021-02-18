pub mod entity;
pub mod handlers;
pub mod modules;

use std::env;
use std::error::Error;
use std::sync::{ Arc };

use entity::client::Client;

use handlers::CommandHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    kankyo::init().expect("Failed to load enviroment!");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the enviroment!");

    env_logger::init();

    let command_handler = CommandHandler {
        prefix: "vex!".to_string()
    };

    let client = Client::new(token, Arc::new(command_handler)).await;

    client.start().await.expect("Failed to start client!");

    Ok(())

}
