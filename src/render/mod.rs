use crossterm::event::KeyEvent;
use std::collections::HashMap;

use crate::{
    config::Config,
    engine::{Engine, EngineEvent, LayoutNode, SplitDir, WindowId},
    render::{
        screen_buffer::ScreenBuffer,
        windows::{info::InfoWindow, table::TableWindow, text::TextWindow},
    },
};
pub mod helpers;
pub mod screen_buffer;
pub mod styling;
pub mod windows;

pub struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
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
        let (cols, rows) = crossterm::terminal::size().expect("could not get size");
        let layout = engine.layout.clone();
        let rect = Rect {
            x: 0,
            y: 0,
            width: cols as usize,
            height: rows as usize,
        };
        self.draw_layout_node(engine, &rect, &layout);
        self.draw_popups(engine, &rect);
        self.screen_buffer.flush();
    }

    pub fn draw_popups(&mut self, engine: &mut Engine, rect: &Rect) {
        if let Some(popup) = engine.popups.clone() {
            let popup_rect = match popup.position {
                crate::engine::PopupPosition::TopRight => Rect {
                    x: rect.width - popup.width,
                    y: 0,
                    width: popup.width,
                    height: popup.height,
                },
                crate::engine::PopupPosition::TopLeft => Rect {
                    x: 0,
                    y: 0,
                    width: popup.width,
                    height: popup.height,
                },
                crate::engine::PopupPosition::BottomRight => Rect {
                    x: rect.width - popup.width,
                    y: 0,
                    width: popup.width,
                    height: popup.height,
                },
                crate::engine::PopupPosition::BottonLeft => Rect {
                    x: 0,
                    y: rect.height - popup.height,
                    width: popup.width,
                    height: popup.height,
                },
                crate::engine::PopupPosition::Center => Rect {
                    x: rect.width.div_euclid(2) - popup.width.div_euclid(2),
                    y: rect.height.div_euclid(2) - popup.height.div_euclid(2),
                    width: popup.width,
                    height: popup.height,
                },
            };
            self.draw_layout_node(engine, &popup_rect, &popup.layout);
        }
    }
    pub fn draw_layout_node(&mut self, engine: &mut Engine, rect: &Rect, node: &LayoutNode) {
        match node {
            LayoutNode::Leaf(win_id) => {
                let win = &self.windows[win_id];
                win.draw(rect, engine, &mut self.screen_buffer);
            }
            LayoutNode::Split {
                direction,
                ratio,
                first,
                second,
            } => {
                let (rect_1, rect_2) = match direction {
                    SplitDir::Vert => {
                        let split_value = (rect.height as f32 * ratio) as usize;
                        (
                            Rect {
                                x: rect.x,
                                y: rect.y,
                                width: rect.width,
                                height: split_value,
                            },
                            Rect {
                                x: rect.x,
                                y: split_value,
                                width: rect.width,
                                height: rect.height - split_value,
                            },
                        )
                    }
                    SplitDir::Horz => {
                        let split_value = (rect.width as f32 * ratio) as usize;
                        (
                            Rect {
                                x: rect.x,
                                y: rect.y,
                                height: rect.height,
                                width: split_value,
                            },
                            Rect {
                                x: split_value,
                                y: rect.x,
                                height: rect.height,
                                width: rect.width - split_value,
                            },
                        )
                    }
                };
                self.draw_layout_node(engine, &rect_1, first);
                self.draw_layout_node(engine, &rect_2, second);
            }
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
            _ => {}
        }
    }
}

pub trait Window {
    fn draw(&self, rect: &Rect, engine: &mut Engine, buffer: &mut ScreenBuffer);
    fn handle_key(&mut self, key: KeyEvent, engine: &mut Engine);
}
