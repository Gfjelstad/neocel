use crate::{
    commands::Key,
    engine::{Engine, WindowState},
};

pub mod spreadsheet;
pub mod text;
pub trait InsertModeProvider {
    fn handle_key(&mut self, window: &mut WindowState, key: Key) -> Result<(), String>;
}
pub trait DocumentDataProvider {
    fn new(data: &str) -> Self;
}
