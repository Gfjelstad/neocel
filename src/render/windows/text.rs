use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    engine::{Engine, WindowId, document::DocumentData},
    render::{Rect, ScreenBuffer, Window, helpers::draw_border},
};

pub struct TextWindow {
    pub window_id: WindowId,
}

impl Window for TextWindow {
    fn draw(&self, rect: &Rect, engine: &mut Engine, buffer: &mut ScreenBuffer) {
        // Draw the border first

        // Get window state and document
        let window = &engine.windows[&self.window_id];
        let doc = &engine.docs[&window.doc_id];
        let rect = draw_border(rect, buffer, self.window_id == engine.active_window.clone());

        if let DocumentData::Text(lines) = &doc.data {
            let bg = engine.config.get_style_color("background", None);
            let fg = engine.config.get_style_color("foreground", None);
            let cursor_row = window.cursor_row + 1; // for border
            let cursor_col = window.cursor_col;

            // Iterate over each cell in the window rect

            for row in 0..rect.height as usize {
                let line_idx = row + rect.y as usize;

                for col in 0..rect.width as usize {
                    let col_idx = col + rect.x as usize;

                    let cell = &mut buffer.cells[line_idx][col_idx];
                    cell.bg = bg;
                    cell.fg = fg;

                    if let Some(line) = lines.get(row) {
                        let chars: Vec<char> = line.chars().collect();
                        cell.ch = chars.get(col).copied().unwrap_or(' ');
                    } else {
                        cell.ch = ' ';
                    }

                    // Cursor highlight
                    if line_idx == cursor_row
                        && col_idx == rect.x as usize + cursor_col
                        && self.window_id == engine.active_window
                    {
                        std::mem::swap(&mut cell.bg, &mut cell.fg);
                    }
                }
            }
        }
    }
    fn handle_key(&mut self, key: KeyEvent, engine: &mut Engine) {
        let (win, doc) = engine.get_current_window(); // Get the document the window is displaying

        // Make sure it's a TextDocument
        let lines = match &mut doc.data {
            DocumentData::Text(text) => text,
            _ => return,
        };

        // Shortcut to cursor state
        let cursor_row = &mut win.cursor_row;
        let cursor_col = &mut win.cursor_col;

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
        }
    }
}
