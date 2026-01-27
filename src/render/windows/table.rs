use std::{
    cmp::{max_by, min},
    collections::HashMap,
    hash::Hash,
};

use crossterm::event::KeyEvent;

use crate::{
    config::blend, engine::{Engine, WindowId, document::DocumentData}, render::{Rect, ScreenBuffer, Window, helpers::draw_border, screen_buffer::Alignment}
};

pub struct TableWindow {
    pub window_id: WindowId,
}
impl Window for TableWindow {
    fn draw(&self, rect: &Rect, engine: &mut Engine, buffer: &mut ScreenBuffer) {
        let fg = engine.config.theme.foreground;
        let bg = engine.config.theme.background;
        let sel_bg = engine.config.theme.selection_background;
        let sel_fg = engine.config.theme.selection_foreground;
        let bg_secondary = blend(bg,fg,0.90);
        let (win, doc) = engine.get_window(&self.window_id);
        let rect = draw_border(
            &self.window_id,
            rect,
            buffer,
            self.window_id == win.id,
            win.border_style,
        );

        if let DocumentData::SpreadSheet(data) = &doc.data {
            let (selected_row, selected_col) = (win.cursor_row, win.cursor_col);
            let scroll_row = &mut win.scroll_rows;
            let scroll_col = &mut win.scroll_cols;
            let max_rows = data.cells.keys().max().copied().unwrap_or(0);
            if (scroll_row.clone() + (rect.height - 1) /* for header */) <= selected_row {
                *scroll_row = selected_row - (rect.height - 2);
            } else if scroll_row.clone() > selected_row {
                *scroll_row = selected_row;
            }

            

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
                col_widths.insert(col, 10.max(max_width + 4));
            }

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
                let chars = ScreenBuffer::format_cell(id.as_str(), size, Alignment::Center);
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

            for row in *scroll_row..=max_rows.min(*scroll_row + rect.height) {
                let buf_y = (row + rect.y + 1) - *scroll_row;
                let color = if buf_y % 2 == 0 { bg } else { bg_secondary };


                let mut loc: usize = rect.x;
                buffer.cells[buf_y][loc].ch = ' ';
                buffer.cells[buf_y][loc].bg = fg;
                
                
                loc += 1;
                for col in 0..=max_cols {
                    let size = col_widths[&col];

                    let mut raw: String = String::new();
                    if let Some(row) = data.cells.get(&(row - *scroll_row))
                        && let Some(col) = row.get(&col)
                    {
                        raw = col.raw.clone();
                    }

                    let chars = ScreenBuffer::format_cell(raw.as_str(), size, Alignment::Center);
                    for buf_idx in loc..loc + size {
                        let cell = &mut buffer.cells[buf_y][buf_idx];
                        cell.ch = chars[buf_idx - loc];
                        cell.bg = color;
                        if row == selected_row && col == selected_col {
                            cell.bg = sel_bg;
                            cell.fg = sel_fg;
                        }
                    }

                    buffer.cells[buf_y][loc].ch = '⎸';
                    buffer.cells[buf_y][loc].bg = color;
                    if row == selected_row && col == selected_col {
                            buffer.cells[buf_y][loc].bg = sel_bg;
                            buffer.cells[buf_y][loc].fg = sel_fg;
                        }
                    // if row == selected_row && col == selected_col {
                    //     buffer.cells[buf_y][loc]
                    //         .attrs
                    //         .push(crossterm::style::Attribute::Bold);
                    // }
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


