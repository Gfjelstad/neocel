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
        let bg = engine.config.theme.background;
        let fg = engine.config.theme.foreground;
        // Get window state and document
        let (window, doc) = engine.get_window(&self.window_id);
        let focussed = self.window_id == window.id;
        let rect = draw_border(&self.window_id, rect, buffer, focussed, window.border_style);

        if let DocumentData::Text(lines) = &doc.data {
            let cursor_row = window.cursor_row; // for border
            let cursor_col = window.cursor_col;

            // Iterate over each cell in the window rect

            for row in 0..rect.height as usize {
                let line_idx = row + rect.y as usize;

                for col in 0..rect.width as usize {
                    let col_idx = col + rect.x as usize;

                    let cell = &mut buffer.cells[line_idx][col_idx];
                    cell.bg = bg;
                    cell.fg = fg;

                    if let Some(line) = lines.data.get(row) {
                        let chars: Vec<char> = line.chars().collect();
                        cell.ch = chars.get(col).copied().unwrap_or(' ');
                    } else {
                        cell.ch = ' ';
                    }

                    // Cursor highlight
                    if row == cursor_row && col_idx == rect.x as usize + cursor_col && focussed {
                        std::mem::swap(&mut cell.bg, &mut cell.fg);
                    }
                }
            }
        }
    }
}
