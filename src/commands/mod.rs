pub mod command_dispatcher;
pub mod globals;

use std::collections::HashMap;

use crossterm::event::{KeyCode as CtKey, KeyEvent, KeyModifiers};

use bitflags::bitflags;
use serde::Deserialize;

use crate::{commands::command_dispatcher::CommandDispatcher, input::{Token, keymaps::ActionNode}};
// pub trait CommandRegistry {
//     fn register_commands(dispatcher: &mut CommandDispatcher) -> Result<(), String>;
// }
