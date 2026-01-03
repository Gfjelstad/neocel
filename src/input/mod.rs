use crate::commands::command_dispatcher::Command;

pub mod input_engine;
pub mod keymaps;
pub type Operator = String;
pub type Motion = String;

#[derive(Clone)]
pub enum Token {
    Digit(u32),
    Operator(Operator),
    Motion(Motion),
    Command(Command),
}
