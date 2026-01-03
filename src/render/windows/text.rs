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
        let rect = draw_border(
            &self.window_id,
            rect,
            buffer,
            self.window_id == engine.active_window.clone(),
        );

        if let DocumentData::Text(lines) = &doc.data {
            let bg = engine.config.get_style_color("background", None);
            let fg = engine.config.get_style_color("foreground", None);
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
                    if row == cursor_row
                        && col_idx == rect.x as usize + cursor_col
                        && self.window_id == engine.active_window
                    {
                        std::mem::swap(&mut cell.bg, &mut cell.fg);
                    }
                }
            }
        }
    }
}
