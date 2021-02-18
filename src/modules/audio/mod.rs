use crate::entity::module::Module;
use crate::entity::command::Command;

mod commands;

use std::sync::Arc;
use std::collections::HashMap;

use commands::*;

pub struct AudioModule {
    commands: HashMap<String, Arc<dyn Command + Send + Sync>>,
    enabled: bool
}

impl Default for AudioModule {
    fn default() -> Self {
        AudioModule {
            commands: {
                let mut list: HashMap<String, Arc<dyn Command + Send + Sync>> = HashMap::new();

                list
            },
            enabled: true
        }
    }
}

impl Module for AudioModule {

    fn name(&self) -> String {
        "audio".to_string()
    }

    fn description(&self) -> String {
        "This module contains all audio-related commands".to_string()
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
