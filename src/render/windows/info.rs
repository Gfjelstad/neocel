use crossterm::event::KeyEvent;

use crate::{
    engine::{Engine, WindowId, document::DocumentData},
    render::{Rect, ScreenBuffer, Window, helpers::draw_border},
};

pub struct InfoWindow {
    pub window_id: WindowId,
}
impl Window for InfoWindow {
    fn draw(&self, rect: &Rect, engine: &mut Engine, buffer: &mut ScreenBuffer) {
        // Draw border; focused if this window is active
        let fg = engine.config.theme.foreground;
        let bg = engine.config.theme.background;
        let (win, doc) = engine.get_window(&self.window_id);
        let inner_rect = draw_border(
            &self.window_id,
            rect,
            buffer,
            self.window_id == win.id,
            win.border_style,
        );

        // Get the document (string)

        let content = match &doc.data {
            DocumentData::Text(data) => data.data.join("\n"), // if you stored TextDocument as Vec<String>
            DocumentData::Help(info) => info.clone(),         // for single String
            _ => return,
        };

        let width = inner_rect.width as usize;
        let height = inner_rect.height as usize;

        // Word wrap content into lines
        let mut lines: Vec<String> = Vec::new();
        for paragraph in content.lines() {
            let mut start = 0;
            let chars: Vec<char> = paragraph.chars().collect();
            while start < chars.len() {
                let end = (start + width).min(chars.len());
                lines.push(chars[start..end].iter().collect());
                start += width;
            }
            if chars.is_empty() {
                lines.push(String::new());
            }
        }

        // Compute vertical offset for centering
        let total_lines = lines.len();
        let v_offset = if total_lines < height {
            (height - total_lines) / 2
        } else {
            0
        };

        // Fill the buffer with spaces and then draw text
        for row in 0..height {
            let buffer_row = row + inner_rect.y as usize;
            for col in 0..width {
                let buffer_col = col + inner_rect.x as usize;
                let cell = &mut buffer.cells[buffer_row][buffer_col];
                cell.ch = ' ';
                cell.fg = fg;
                cell.bg = bg;
                cell.attrs.clear();
            }
        }

        // Draw wrapped lines
        for (i, line) in lines.iter().enumerate() {
            let row = i + v_offset;
            if row >= height {
                break;
            }
            let buffer_row = row + inner_rect.y as usize;
            for (col, ch) in line.chars().enumerate() {
                if col >= width {
                    break;
                }
                let buffer_col = col + inner_rect.x as usize;
                let cell = &mut buffer.cells[buffer_row][buffer_col];
                cell.ch = ch;
                cell.fg = fg;
                cell.bg = bg;
                cell.attrs.clear();
            }
        }
    }
}
