use std::collections::btree_map::Range;

use serde::Serialize;

use crate::{
    commands::KeyCode,
    engine::{
        WindowState,
        document::DocumentData,
        documents::{DocumentDataProvider, InsertModeProvider},
    },
};

#[derive(Serialize)]
pub struct TextDocumentData {
    pub data: Vec<String>,
    pub selected: Option<((usize, usize), (usize, usize))>,
}
impl DocumentDataProvider for TextDocumentData {
    fn new(data: &str) -> Self {
        Self {
            data: data.lines().map(|s| s.to_string()).collect(),
            selected: None,
        }
    }
}
impl InsertModeProvider for TextDocumentData {
    fn handle_key(
        &mut self,
        window: &mut WindowState,
        key: crate::commands::Key,
    ) -> Result<(), String> {
        // Make sure it's a TextDocument
        // Shortcut to cursor state
        let cursor_row = &mut window.cursor_row;
        let cursor_col = &mut window.cursor_col;
        let lines = &mut self.data;

        match key.code {
            // Insert character
            KeyCode::Char(c) => {
                if *cursor_row >= lines.len() {
                    lines.push(String::new());
                }
                let line = &mut lines[*cursor_row];
                if *cursor_col > line.len() {
                    *cursor_col = line.len();
                }
                line.insert(*cursor_col, c);
                *cursor_col += 1;
            }

            // New line
            KeyCode::Enter => {
                let line = &mut lines[*cursor_row];
                let remainder = line.split_off(*cursor_col);
                lines.insert(*cursor_row + 1, remainder);
                *cursor_row += 1;
                *cursor_col = 0;
            }

            // Backspace
            KeyCode::Backspace => {
                if *cursor_col > 0 {
                    let line = &mut lines[*cursor_row];
                    line.remove(*cursor_col - 1);
                    *cursor_col -= 1;
                } else if *cursor_row > 0 {
                    let current = lines.remove(*cursor_row);
                    *cursor_row -= 1;
                    let prev = &mut lines[*cursor_row];
                    *cursor_col = prev.len();
                    prev.push_str(&current);
                }
            }

            // Cursor movement
            KeyCode::Left => {
                if *cursor_col > 0 {
                    *cursor_col -= 1;
                } else if *cursor_row > 0 {
                    *cursor_row -= 1;
                    *cursor_col = lines[*cursor_row].len();
                }
            }

            KeyCode::Right => {
                if *cursor_col < lines[*cursor_row].len() {
                    *cursor_col += 1;
                } else if *cursor_row + 1 < lines.len() {
                    *cursor_row += 1;
                    *cursor_col = 0;
                }
            }

            KeyCode::Up => {
                if *cursor_row > 0 {
                    *cursor_row -= 1;
                    *cursor_col = (*cursor_col).min(lines[*cursor_row].len());
                }
            }

            KeyCode::Down => {
                if *cursor_row + 1 < lines.len() {
                    *cursor_row += 1;
                    *cursor_col = (*cursor_col).min(lines[*cursor_row].len());
                }
            }

            _ => {}
        };
        Ok(())
    }
}
