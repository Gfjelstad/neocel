use crossterm::event::KeyEvent;

use crate::{
    engine::{Engine, WindowId},
    render::{Rect, ScreenBuffer, Window, helpers::draw_border},
};

pub struct TableWindow {
    pub window_id: WindowId,
}
impl Window for TableWindow {
    fn draw(&self, rect: &Rect, engine: &mut Engine, buffer: &mut ScreenBuffer) {
        draw_border(
            &self.window_id,
            rect,
            buffer,
            self.window_id == engine.active_window.clone(),
        );
        let _ = &engine.docs[&engine.windows[&self.window_id].doc_id];
    }
    fn handle_key(&mut self, _key: KeyEvent, _engine: &mut Engine) {}
}
