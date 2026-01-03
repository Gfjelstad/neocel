use std::{
    cmp::{max_by, min},
    collections::HashMap,
    hash::Hash,
};

use crossterm::event::KeyEvent;

use crate::{
    engine::{Engine, WindowId, document::DocumentData},
    render::{Rect, ScreenBuffer, Window, helpers::draw_border, styling::hex_to_color},
};

pub struct TableWindow {
    pub window_id: WindowId,
}
impl Window for TableWindow {
    fn draw(&self, rect: &Rect, engine: &mut Engine, buffer: &mut ScreenBuffer) {
        let mut rect = draw_border(
            &self.window_id,
            rect,
            buffer,
            self.window_id == engine.active_window.clone(),
        );

        let doc = &engine.docs[&engine.windows[&self.window_id].doc_id];
        if let DocumentData::SpreadSheet(data) = &doc.data {
            let max_rows = data.cells.keys().max().copied().unwrap_or(0);
            let max_cols = data
                .cells
                .values()
                .flat_map(|r| r.keys())
                .max()
                .copied()
                .unwrap_or(0);

            let mut col_widths: HashMap<usize, usize> = HashMap::new();
            for col in 0..=max_cols {
                let max_width = (0..max_rows)
                    .filter_map(|row| data.cells.get(&row)?.get(&col).map(|s| s.raw.len()))
                    .max()
                    .unwrap_or(3);
                col_widths.insert(col, vec![10, max_width + 4].into_iter().max().unwrap());
            }
            let bg =
                hex_to_color(engine.config.styles.get("background").unwrap().as_str()).unwrap();
            let fg =
                hex_to_color(engine.config.styles.get("foreground").unwrap().as_str()).unwrap();
            let bg_secondary = hex_to_color(
                engine
                    .config
                    .styles
                    .get("background_secondary")
                    .unwrap()
                    .as_str(),
            )
            .unwrap();
            // render col ids
            let mut loc: usize = rect.x;

            buffer.cells[rect.y][loc].ch = ' ';
            buffer.cells[rect.y][loc].bg = fg;
            buffer.cells[rect.y][loc].fg = bg;

            buffer.cells[rect.y][loc]
                .attrs
                .push(crossterm::style::Attribute::Bold);

            loc += 1;
            for col in 0..=max_cols {
                let size = col_widths[&col];
                let id = column_num_to_id(col);
                let chars = format_cell(id.as_str(), size, Alignment::Center);
                for buf_idx in loc..loc + size {
                    let cell = &mut buffer.cells[rect.y][buf_idx];
                    cell.ch = chars[buf_idx - loc];
                    cell.bg = fg;
                    cell.fg = bg;
                    cell.attrs.push(crossterm::style::Attribute::Bold);
                }
                buffer.cells[rect.y][loc].ch = '⎸';
                buffer.cells[rect.y][loc]
                    .attrs
                    .push(crossterm::style::Attribute::Bold);
                buffer.cells[rect.y][loc].bg = fg;
                buffer.cells[rect.y][loc].fg = bg;
                loc += size;
            }

            for row in 0..=max_rows {
                let buf_y = row + rect.y + 1;
                let color = if row % 2 == 0 { bg } else { bg_secondary };
                let mut loc: usize = rect.x;
                buffer.cells[buf_y][loc].ch = '⎸';
                buffer.cells[buf_y][loc].bg = fg;
                loc += 1;
                for col in 0..=max_cols {
                    let size = col_widths[&col];

                    let mut raw: String = String::new();
                    if let Some(row) = data.cells.get(&row)
                        && let Some(col) = row.get(&col)
                    {
                        raw = col.raw.clone();
                    }

                    let chars = format_cell(raw.as_str(), size, Alignment::Center);
                    for buf_idx in loc..loc + size {
                        let cell = &mut buffer.cells[buf_y][buf_idx];
                        cell.ch = chars[buf_idx - loc];
                        cell.bg = color;
                    }
                    buffer.cells[buf_y][loc].ch = '⎸';
                    buffer.cells[buf_y][loc].bg = color;
                    loc += size;
                }
            }
        }
    }
}

fn column_num_to_id(mut col: usize) -> String {
    let mut result = String::new();

    loop {
        result.push((b'A' + (col % 26) as u8) as char);
        if col < 26 {
            break;
        }
        col = col / 26 - 1;
    }

    result.chars().rev().collect()
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

fn format_cell(content: &str, width: usize, align: Alignment) -> Vec<char> {
    let mut chars = Vec::with_capacity(width);

    // Truncate if too long
    let content: String = content.chars().take(width).collect();
    let content_len = content.len();

    if content_len >= width {
        return content.chars().collect();
    }

    let padding = width - content_len;

    match align {
        Alignment::Left => {
            chars.extend(content.chars());
            chars.extend(std::iter::repeat(' ').take(padding));
        }
        Alignment::Right => {
            chars.extend(std::iter::repeat(' ').take(padding));
            chars.extend(content.chars());
        }
        Alignment::Center => {
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            chars.extend(std::iter::repeat(' ').take(left_pad));
            chars.extend(content.chars());
            chars.extend(std::iter::repeat(' ').take(right_pad));
        }
    }

    chars
}
