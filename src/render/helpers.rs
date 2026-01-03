use crossterm::style::Attribute;
use serde::{Deserialize, Serialize};

use crate::render::{Rect, screen_buffer::ScreenBuffer};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BorderStyle {
    Single,
    Double,
    Rounded,
    Shadow,
    None,
}

fn border_chars(style: BorderStyle) -> (char, char, char, char, char, char) {
    match style {
        BorderStyle::Single => ('─', '│', '┌', '┐', '└', '┘'),
        BorderStyle::Double => ('═', '║', '╔', '╗', '╚', '╝'),
        BorderStyle::Rounded => ('─', '│', '╭', '╮', '╰', '╯'),
        BorderStyle::Shadow => (' ', ' ', ' ', ' ', ' ', ' '), // Shadows are usually spaces
        BorderStyle::None => (' ', ' ', ' ', ' ', ' ', ' '),   // No border
    }
}
pub fn draw_border(
    win_id: &String,
    rect: &Rect,
    buffer: &mut ScreenBuffer,
    focused: bool,
    style: Option<BorderStyle>,
) -> Rect {
    let style = style.unwrap_or(BorderStyle::Single);
    // Decide which characters to use
    let (h_line, v_line, tl, tr, bl, br) = border_chars(style);
    let attrs = if focused {
        vec![Attribute::Bold]
    } else {
        vec![]
    };

    // Top and bottom horizontal lines
    for col in rect.x + 1..rect.x + rect.width - 1 {
        let x = col;
        let y_top = rect.y;
        let y_bot = rect.y + rect.height - 1;

        if x >= 4 && x < 4 + win_id.len() {
            buffer.cells[y_top][x].ch = win_id.chars().nth(x - 4).unwrap_or(' ');
            buffer.cells[y_top][x].attrs = attrs.clone();
        } else {
            buffer.cells[y_top][x].ch = h_line;
            buffer.cells[y_top][x].attrs = attrs.clone();
        }

        buffer.cells[y_bot][x].ch = h_line;
        buffer.cells[y_bot][x].attrs = attrs.clone();
    }

    // Left and right vertical lines
    for row in rect.y + 1..rect.y + rect.height - 1 {
        let y = row;
        let x_left = rect.x;
        let x_right = rect.x + rect.width - 1;

        buffer.cells[y][x_left].ch = v_line;
        buffer.cells[y][x_left].attrs = attrs.clone();

        buffer.cells[y][x_right].ch = v_line;
        buffer.cells[y][x_right].attrs = attrs.clone();
    }

    // Corners
    let l = rect.x;
    let r = rect.x + rect.width - 1;
    let t = rect.y;
    let b = rect.y + rect.height - 1;

    buffer.cells[t][l].ch = tl;
    buffer.cells[t][l].attrs = attrs.clone();

    buffer.cells[t][r].ch = tr;
    buffer.cells[t][r].attrs = attrs.clone();

    buffer.cells[b][l].ch = bl;
    buffer.cells[b][l].attrs = attrs.clone();

    buffer.cells[b][r].ch = br;
    buffer.cells[b][r].attrs = attrs.clone();

    // Return inner rect (inside border)
    Rect {
        x: rect.x + 1,
        y: rect.y + 1,
        width: rect.width - 2,
        height: rect.height - 2,
    }
}
