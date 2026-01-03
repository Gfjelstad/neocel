use std::collections::HashMap;

use serde::Deserialize;
use serde_json::json;

use crate::{
    api::{APIMethod, APIMethodParams, APIMethodResult, APIRegister, utils},
    engine::{EngineEvent, layout::LayoutNode},
};

pub struct EngineAPI {}

impl EngineAPI {
    pub fn create_window(state: &mut APIMethodParams) -> APIMethodResult {
        Ok(None)
    }
    pub fn get_current_window(state: &mut APIMethodParams) -> APIMethodResult {
        let win_id = state.engine.active_window.clone();
        let win = state.engine.windows.get(&win_id).unwrap();
        let doc = state.engine.docs.get(&win.doc_id.clone()).unwrap();
        Ok(Some(json!({
            "window": win,
            "document": doc
        })))
    }
    pub fn close_window(state: &mut APIMethodParams) -> APIMethodResult {
        let params = utils::try_parse::<CloseWindowParams>(state.params.clone())?;
        let window_id = &params.win_id;
        state.engine.windows.remove(window_id);
        if let Some(old_layout) = std::mem::take(&mut state.engine.layout) {
            let new_layout = old_layout.remove_window(window_id).unwrap_or_else(|| {
                let new_win = state.engine.create_empty_window();
                LayoutNode::Leaf(new_win)
            });
            state.engine.layout = Some(new_layout);
        }
        if &state.engine.active_window == window_id {
            state.engine.active_window = state.engine.windows.keys().next().unwrap().clone();
        }
        state
            .engine
            .emit(&EngineEvent::WindowClose(window_id.clone()));
        Ok(None)
    }
}

impl APIRegister for EngineAPI {
    fn register_methods(&mut self, api: &mut super::API) {
        let mut methods: HashMap<&str, APIMethod> = HashMap::new();
        methods.insert("window.create", Self::create_window);
        methods.insert("window.get_current", Self::get_current_window);
        methods.insert("window.close", Self::close_window);
    }
}

#[derive(Deserialize)]
struct CloseWindowParams {
    win_id: String,
}
