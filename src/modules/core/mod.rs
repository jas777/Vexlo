use crate::entity::module::Module;
use crate::entity::command::Command;

mod commands;

use std::sync::Arc;
use std::collections::HashMap;

use commands::*;

pub struct CoreModule {
    commands: HashMap<String, Arc<dyn Command + Send + Sync>>,
    enabled: bool
}

impl Default for CoreModule {
    fn default() -> Self {
        CoreModule {
            commands: {
                let mut list: HashMap<String, Arc<dyn Command + Send + Sync>> = HashMap::new();

                list.insert("ping".to_string(), Arc::new(PingCommand));
                list.insert("help".to_string(), Arc::new(HelpCommand));

                list
            },
            enabled: true
        }
    }
}

impl Module for CoreModule {

    fn name(&self) -> String {
        "core".to_string()
    }

    fn description(&self) -> String {
        "This module contains all basic commands".to_string()
    }

    fn commands(&self) -> HashMap<String, Arc<dyn Command + Send + Sync>> {
        self.commands.clone()
    }

    fn enabled(&self) -> bool {
        self.enabled.clone()
    }

    fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

}
