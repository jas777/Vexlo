use crate::entity::client::Client;
use crate::entity::command::Command;

use twilight::model::channel::Message;

use async_trait::async_trait;

use std::error::Error;
use std::sync::Arc;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {

    fn name(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "Pong!".to_string()
    }

    fn usage(&self) -> String {
        "vex!ping".to_string()
    }

    fn example(&self) -> String {
        "vex!ping".to_string()
    }

    fn cooldown(&self) -> i32 {
        10
    }

    // fn user_permissions(&self) -> Permissions {
        
    // }

    // fn bot_permissions(&self) -> Permissions {
        
    // }

    async fn execute(&self, client: Arc<Client>, _args: Vec<String>, msg: &Message) -> Result<(), Box<dyn Error + Send + Sync>> {

        client.http.create_message(msg.channel_id).content("Pong!")?.await?;

        Ok(())

    }

}