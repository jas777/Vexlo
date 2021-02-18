use crate::entity::command::Command;
use crate::entity::client::Client;

use std::sync::Arc;
use std::error::Error;

use twilight::model::channel::Message;

use async_trait::async_trait;

pub struct JoinCommand;

#[async_trait]
impl Command for JoinCommand {

    fn name(&self) -> String {
        "join".to_string()
    }

    fn description(&self) -> String {
        "Summons the bot in the voice channel you're in".to_string()
    }

    fn usage(&self) -> String {
        "join".to_string()
    }

    fn example(&self) -> String {
        "vex!join".to_string()
    }

    fn cooldown(&self) -> i32 {
        10
    }

    // fn user_permissions(&self) -> Permissions;

    // fn bot_permissions(&self) -> Permissions;

    async fn execute(&self, _client: Arc<Client>, _args: Vec<String>, _msg: &Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

}