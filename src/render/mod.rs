use std::collections::HashMap;

use crate::engine::{Engine, EngineEvent, WindowId};

pub struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

pub struct UI {
    pub windows: HashMap<WindowId, Box<dyn Window>>,
}
impl UI {
    fn new() -> Self {
        Self {
            windows: HashMap::new(),
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
                    crate::engine::DocType::SpreadSheet => {
                        Box::new(TableWindow { window_id: win_id })
                    }
                    crate::engine::DocType::Info => Box::new(InfoWindow { window_id: win_id }),
                    crate::engine::DocType::Text => Box::new(TextWindow { window_id: win_id }),
                };
            }
            _ => {}
        }
    }
}

pub trait Window {
    fn draw(&self, rect: Rect, engine: &mut Engine, buffer: &mut ScreenBuffer);
    fn handle_key(&mut self, key: u32, engine: &mut Engine);
}

pub struct TableWindow {
    window_id: WindowId,
}
impl Window for TableWindow {
    fn draw(&self, rect: Rect, engine: &mut Engine, buffer: &mut ScreenBuffer) {
        println!("should draw table window");
        let doc = &engine.docs[&engine.windows[&self.window_id].doc_id];
    }
    fn handle_key(&mut self, key: u32, engine: &mut Engine) {}
}

pub struct InfoWindow {
    window_id: WindowId,
}
impl Window for InfoWindow {
    fn draw(&self, rect: Rect, engine: &mut Engine, buffer: &mut ScreenBuffer) {
        println!("should draw info");
        let doc = &engine.docs[&engine.windows[&self.window_id].doc_id];
    }
    fn handle_key(&mut self, key: u32, engine: &mut Engine) {}
}
pub struct TextWindow {
    window_id: WindowId,
}
impl Window for TextWindow {
    fn draw(&self, rect: Rect, engine: &mut Engine, buffer: &mut ScreenBuffer) {
        println!("should draw text window");
        let doc = &engine.docs[&engine.windows[&self.window_id].doc_id];
    }
    fn handle_key(&mut self, key: u32, engine: &mut Engine) {}
}
pub struct ScreenBuffer {
    width: u16,
    height: u16,
    // cells: Vec<Cell>,
}
