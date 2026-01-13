use crate::commands::command_dispatcher::CommandRequest;

pub mod input_engine;
pub mod keymaps;
pub type Operator = String;
pub type Motion = String;

#[derive(Clone, Debug)]
pub enum Token {
    Digit(u32),
    Operator(Operator),
    Motion(Motion),
    Command(CommandRequest),
}
