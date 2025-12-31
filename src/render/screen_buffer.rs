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

#[derive(Clone)]
pub struct BufferCell {
    pub ch: char,              // the character to display
    pub fg: Color,             // foreground color
    pub bg: Color,             // background color
    pub attrs: Vec<Attribute>, // d, underline, etc
}
