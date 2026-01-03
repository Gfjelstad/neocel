use crossterm::event::KeyEvent;
use std::collections::HashMap;

use crate::{
    config::Config,
    engine::{Engine, EngineEvent, WindowId, layout::LayoutNode, popup::RelativeTo},
    render::{
        screen_buffer::ScreenBuffer,
        windows::{info::InfoWindow, table::TableWindow, text::TextWindow},
    },
};
pub mod helpers;
pub mod screen_buffer;
pub mod styling;
pub mod windows;

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct UI {
    pub windows: HashMap<WindowId, Box<dyn Window>>,
    pub screen_buffer: ScreenBuffer,
}
impl UI {
    pub fn new(config: &Config) -> Self {
        let size = crossterm::terminal::size().expect("could not get size");
        Self {
            windows: HashMap::new(),
            screen_buffer: ScreenBuffer::new(size.0, size.1, config),
        }
    }

    pub fn draw(&mut self, engine: &mut Engine) {
        if engine.layout.is_none() {
            return;
        }
        let (cols, rows) = crossterm::terminal::size().expect("could not get size");
        let layout = engine.layout.clone();
        let rect = Rect {
            x: 0,
            y: 0,
            width: cols as usize,
            height: rows as usize,
        };
        self.draw_layout_node(engine, &rect, &layout.unwrap());
        self.draw_popups(engine, &rect);
        self.screen_buffer.flush();
    }

    pub fn draw_popups(&mut self, engine: &mut Engine, rect: &Rect) -> Result<(), String> {
        let layout = engine.layout.as_ref().ok_or("Engine layout not found")?;
        let wins: HashMap<String, Rect> = layout.get_rects(rect).into_iter().collect();

        if let Some(popup) = &mut engine.popups.clone() {
            let relative_to = popup.relative_to.clone();

            let rect = match relative_to {
                RelativeTo::Editor => Ok(Rect {
                    width: popup.width,
                    height: popup.height,
                    x: 0,
                    y: 0,
                }),
                RelativeTo::Win(win_id) => wins
                    .get(&win_id)
                    .cloned()
                    .ok_or_else(|| "window not found".to_string()),
                RelativeTo::Cursor => {
                    let win = engine.get_current_window().0;
                    wins.get(&win.id)
                        .map(|curr_win| Rect {
                            width: popup.width,
                            height: popup.height,
                            x: curr_win.x + win.cursor_col,
                            y: curr_win.y + win.cursor_row,
                        })
                        .ok_or_else(|| "failed to get cursor position".to_string())
                }
            }?;

            let popup_rect = popup.get_rect(&rect)?;
            self.draw_layout_node(engine, &popup_rect, &popup.layout);
        }

        Ok(())
    }
    pub fn draw_layout_node(&mut self, engine: &mut Engine, rect: &Rect, node: &LayoutNode) {
        let node_rects = node.get_rects(rect);

        for (window_id, rect) in node_rects {
            let win = &self.windows[&window_id];
            win.draw(&rect, engine, &mut self.screen_buffer);
        }
    }

    pub fn handle_events(&mut self, engine: &mut Engine) {
        let events: Vec<EngineEvent> = engine.events.drain(..).collect();
        for event in events {
            self.handle_event(engine, event);
        }
    }

    fn handle_event(&mut self, engine: &mut Engine, event: EngineEvent) {
        match event {
            EngineEvent::WindowCreate(win_id) => {
                let doc = &engine.docs[&engine.windows[&win_id].doc_id];
                let window: Box<dyn Window> = match doc.doc_type {
                    crate::engine::document::DocType::SpreadSheet => Box::new(TableWindow {
                        window_id: win_id.clone(),
                    }),
                    crate::engine::document::DocType::Info => Box::new(InfoWindow {
                        window_id: win_id.clone(),
                    }),
                    crate::engine::document::DocType::Text => Box::new(TextWindow {
                        window_id: win_id.clone(),
                    }),
                };
                self.windows.insert(win_id, window);
            }
            EngineEvent::WindowDocChange(_window_id, _doc_id) => {}
            EngineEvent::WindowClose(window_id) => {
                self.windows.remove(&window_id);
            }
            _ => {}
        }
    }
}

pub trait Window {
    fn draw(&self, rect: &Rect, engine: &mut Engine, buffer: &mut ScreenBuffer);
}
