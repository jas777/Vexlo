use crate::entity::handler::EventHandler;
use crate::entity::client::Client;
use crate::entity::command::Command;

use std::sync::Arc;
use std::error::Error;

use twilight::model::gateway::payload::MessageCreate;
use twilight::model::gateway::payload::Ready;
use twilight::model::channel::Channel;
use twilight::model::channel::Message;
use twilight::http::Client as HttpClient;


use log::{ log, info, Level };

use async_trait::async_trait;

pub struct CommandHandler {
    pub prefix: String
}

#[async_trait]
impl EventHandler for CommandHandler {

    async fn message_create(&self, client: Arc<Client>, event: Box<MessageCreate>) -> Result<(), Box<dyn Error + Send + Sync>> {

        let http: &HttpClient = &client.http;

        if let Channel::Guild(_channel) = http.channel(event.channel_id).await.unwrap().unwrap() {
            
            if event.author.bot {
                return Ok(())
            }

            let msg: Message = event.0 as Message;

            let mut args: Vec<String> = msg
            .content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

            let command: String = args[0].clone().replace(&self.prefix, "");

            let cmd: Arc<dyn Command + Send + Sync> = match client.modules.values().find_map(|m| m.commands().get(&command).cloned()) {
                Some(c) => c,
                None => return Ok(()),
            };

            args.drain(..1);

            tokio::spawn(async move {
                info!("{} executed by {} on {}", &cmd.name(), &msg.author.id, &msg.guild_id.unwrap());
                cmd.execute(client.clone(), args, &msg).await.expect("Command failed!");
            });

        }

        Ok(())

    }

    async fn ready(&self, client: Arc<Client>, _: Box<Ready>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let current_user = client.http.current_user().await.unwrap();

        log!(Level::Info, "Logged in as {}#{}", current_user.name, current_user.discriminator);

        for module in client.modules.keys() {
            log!(Level::Info, "{} loaded", module);
        }

        Ok(())
    }

}
