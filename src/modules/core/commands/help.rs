use crate::entity::client::Client;
use crate::entity::command::Command;
use crate::entity::module::Module;

use twilight::builders::embed::EmbedBuilder;
use twilight::model::channel::Message;

use async_trait::async_trait;

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    fn name(&self) -> String {
        "help".to_string()
    }

    fn description(&self) -> String {
        "Displays information about given command/module".to_string()
    }

    fn usage(&self) -> String {
        "vex!help [command|module]".to_string()
    }

    fn example(&self) -> String {
        "vex!help ping".to_string()
    }

    fn cooldown(&self) -> i32 {
        10
    }

    // fn user_permissions(&self) -> Permissions {
    // }

    // fn bot_permissions(&self) -> Permissions {
    // }

    async fn execute(
        &self,
        client: Arc<Client>,
        args: Vec<String>,
        msg: &Message,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let modules: Vec<&String> = client.modules.keys().collect::<Vec<&String>>();
        let commands: HashMap<String, Arc<dyn Command + Send + Sync>> = {
            let mut map: HashMap<String, Arc<dyn Command + Send + Sync>> = HashMap::new();

            for m in client.modules.values() {
                for c in m.commands() {
                    map.insert(c.0, c.1);
                }
            }

            map
        };

        if args.len() <= 0 {
            let mut desc: String = String::from(
                "Hi! I'm Vexlo, a multipurpose bot. To get more detailed information about a module or a command, run `vex!help [module/command]`\n\nAvailable modules: "
            );

            desc.push_str(
                &modules
                    .iter()
                    .map(|m| format!("`{}`", m))
                    .collect::<Vec<String>>()
                    .join(", "),
            );

            client
                .http
                .create_message(msg.channel_id)
                .embed({
                    EmbedBuilder::new()
                        .title("Vexlo - Help".to_string())
                        .description(desc)
                        .color(0x02FFC9)
                        .build()
                })?
                .await?;

            Ok(())
        } else {
            if modules.contains(&&args[0].to_lowercase()) {
                let module: &Arc<dyn Module + Send + Sync> = client.modules.get(&args[0]).unwrap();

                client
                    .http
                    .create_message(msg.channel_id)
                    .embed({
                        EmbedBuilder::new()
                            .title(module.name())
                            .description(module.description())
                            .add_field(
                                "Available commands",
                                module
                                    .commands()
                                    .keys()
                                    .map(|n| format!("`{}`", n))
                                    .collect::<Vec<String>>()
                                    .join(", "),
                            )
                                .commit()
                            .color(0x02FFC9)
                            .build()
                    })?
                    .await?;

                Ok(())
            } else if commands.get(&args[0]).is_some() {
                let command: &Arc<dyn Command + Send + Sync> = commands.get(&args[0]).unwrap();

                client
                    .http
                    .create_message(msg.channel_id)
                    .embed({
                        EmbedBuilder::new()
                            .title(command.name())
                            .description(command.description())
                            .add_field("Usage", command.usage())
                                .commit()
                            .add_field("Example", command.example())
                                .commit()
                            .color(0x02FFC9)
                            .build()
                        // F009E2
                    })?
                    .await?;

                Ok(())
            } else {
                Ok(())
            }
        }

        // Ok(())
    }
}
