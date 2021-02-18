use crate::entity::client::Client;

use twilight::model::channel::Message;

use async_trait::async_trait;

use std::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait Command {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn usage(&self) -> String;

    fn example(&self) -> String;

    fn cooldown(&self) -> i32;

    // fn user_permissions(&self) -> Permissions;

    // fn bot_permissions(&self) -> Permissions;

    async fn execute(&self, _client: Arc<Client>, _args: Vec<String>, _msg: &Message) -> Result<(), Box<dyn Error + Send + Sync>>;
}
