use crate::entity::command::Command;

use std::sync::Arc;
use std::collections::HashMap;
use std::fmt;

pub trait Module {

    fn name(&self) -> String;

    fn description(&self) -> String;

    fn commands(&self) -> HashMap<String, Arc<dyn Command + Send + Sync>>;

    fn enabled(&self) -> bool;

    fn toggle(&mut self);

}

impl fmt::Debug for dyn Module + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module")
         .field("name", &self.name())
         .finish()
    }
}
