use crate::commands::command_dispatcher::Command;

pub mod input_engine;
pub type Operator = String;
pub type Motion = String;

pub enum Token {
    Digit(u32),
    Operator(Operator),
    Motion(Motion),
    Command(Command),
}
