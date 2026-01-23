use std::io::{Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Attribute, Color, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
};

use crate::{config::Config, render::styling::hex_to_color};

#[derive(Clone)]
pub struct ScreenBuffer {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Vec<BufferCell>>,
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16, config: &Config) -> Self {
        Self {
            width,
            height,
            cells: vec![
                vec![
                    BufferCell {
                        ch: ' ',
                        bg: hex_to_color(config.styles["background"].as_str())
                            .unwrap_or(Color::Black),
                        fg: hex_to_color(config.styles["foreground"].as_str())
                            .unwrap_or(Color::Red),
                        attrs: vec![],
                    };
                    usize::from(width)
                ];
                usize::from(height)
            ],
        }
    }

    pub fn write_str(
        &mut self,
        row: usize,
        start_col: usize,
        content: &str,
        template_cell: BufferCell,
    ) -> (usize, usize) {
        let size = content.len();
        let chars: Vec<char> = content.chars().collect();

        for buf_idx in start_col..start_col + size {
            let mut val = template_cell.clone();
            val.ch = chars[buf_idx - start_col];
            let cell = self.get_cell_mut(row, buf_idx);
            match cell {
                Some(c) => *c = val,
                _ => {}
            }
        }

        return (row, (self.width as usize).min(start_col + size));
    }

    pub fn write_section(
        &mut self,
        row: usize,
        start_col: usize,
        width: usize,
        align: Alignment,
        content: &str,
        template_cell: BufferCell,
    ) -> (usize, usize) {
        let chars = Self::format_cell(content, width, align);
        self.write_str(row, start_col, &chars.iter().collect::<String>(), template_cell)
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut BufferCell> {
        if let Some(row_cells) = self.cells.get_mut(row) {
            row_cells.get_mut(col)
        } else {
            None
        }
    }

    pub fn format_cell(content: &str, width: usize, align: Alignment) -> Vec<char> {
        let mut chars = Vec::with_capacity(width);

        let content_chars: Vec<char> = content.chars().collect();
        let content_len = content_chars.len();

        // If content fits or is too long
        if content_len >= width {
            match align {
                Alignment::Right => {
                    // Take last `width` characters
                    return content_chars[content_len - width..].to_vec();
                }
                _ => {
                    // Take first `width` characters
                    return content_chars[..width].to_vec();
                }
            }
        }

        // Content is shorter than width, add padding
        let padding = width - content_len;

        match align {
            Alignment::Left => {
                chars.extend(content_chars);
                chars.extend(std::iter::repeat_n(' ', padding));
            }
            Alignment::Right => {
                chars.extend(std::iter::repeat_n(' ', padding));
                chars.extend(content_chars);
            }
            Alignment::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                chars.extend(std::iter::repeat_n(' ', left_pad));
                chars.extend(content_chars);
                chars.extend(std::iter::repeat_n(' ', right_pad));
            }
        }

        chars
    }

    pub fn flush(&self) {
        let mut stdout = stdout();

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                stdout.queue(SetForegroundColor(cell.fg)).unwrap();
                stdout.queue(SetBackgroundColor(cell.bg)).unwrap();
                for attr in &cell.attrs {
                    stdout.queue(SetAttribute(*attr)).unwrap();
                }
                print!("{}", cell.ch);
                stdout.queue(ResetColor).unwrap();
            }
        }

        stdout.flush().unwrap();
    }
}
#[derive(Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}
#[derive(Clone)]
pub struct BufferCell {
    pub ch: char,              // the character to display
    pub fg: Color,             // foreground color
    pub bg: Color,             // background color
    pub attrs: Vec<Attribute>, // d, underline, etc
}
