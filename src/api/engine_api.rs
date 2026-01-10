use crate::{
    api::{
        APIMethod, APIMethodParams, APIMethodResult, APIRegister,
        utils::{self, try_parse},
    },
    engine::{
        EngineEvent, SplitDirection, WindowState,
        layout::{LayoutNode, SplitDir},
        popup::{PopupPosition, PopupWindow, RelativeTo},
    },
    render::helpers::BorderStyle,
};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

pub struct EngineAPI {}

impl EngineAPI {
    pub fn create_window(state: &mut APIMethodParams) -> APIMethodResult {
        let params = try_parse::<CreateWindowParams>(state.params.clone())?;
        match params {
            CreateWindowParams::Split {
                doc,
                enter,
                src_win,
                direction,
                border,
                ratio,
            } => {
                if state.engine.layout.is_none() {
                    return Err("Invalid Layout".to_string());
                }
                let (win_id, mut win) = WindowState::new(doc);
                win.border_style = border;
                state.engine.windows.insert(win_id.clone(), win);
                if let Some(old_node) = state
                    .engine
                    .layout
                    .as_mut()
                    .and_then(|l| l.find_child(src_win))
                {
                    let cloned: LayoutNode = old_node.clone();
                    let new_node = LayoutNode::Leaf(win_id.clone());
                    let first: LayoutNode;
                    let second: LayoutNode;
                    let dir: SplitDir;
                    match direction {
                        SplitDirection::Up => {
                            second = cloned;
                            first = new_node;
                            dir = SplitDir::Vert;
                        }
                        SplitDirection::Down => {
                            first = new_node;
                            second = cloned;
                            dir = SplitDir::Vert;
                        }
                        SplitDirection::Left => {
                            first = new_node;
                            second = cloned;
                            dir = SplitDir::Horz;
                        }
                        SplitDirection::Right => {
                            first = cloned;
                            second = new_node;
                            dir = SplitDir::Horz;
                        }
                    }
                    *old_node = LayoutNode::Split {
                        direction: dir,
                        ratio: ratio.unwrap_or(0.5),
                        first: Box::new(first),
                        second: Box::new(second),
                    };
                    if enter {
                        state.engine.active_window = win_id.clone();
                    }
                    state
                        .engine
                        .events
                        .push(EngineEvent::WindowCreate(win_id.clone()));
                }
            }
            CreateWindowParams::Floating {
                doc,
                enter,
                relative,
                position,
                win,
                width,
                height,
                row,
                col,
                style,
                border,
                focusable,
                zindex,
            } => {
                let (win_id, mut window) = WindowState::new(doc.clone());
                window.border_style = border;

                if matches!(relative, RelativeTo::Win(_)) && win.is_none() {
                    return Err("relative = win required a window id \"win\"".to_string());
                }

                state.engine.windows.insert(win_id.clone(), window);
                let relative = if let Some(win) = win {
                    RelativeTo::Win(win)
                } else {
                    relative
                };

                state.engine.popups = Some(PopupWindow {
                    position,
                    relative_to: relative,
                    col,
                    row,
                    width,
                    height,
                    layout: LayoutNode::Leaf(win_id.clone()),
                });
                if enter {
                    state.engine.active_window = win_id.clone();
                }
                state
                    .engine
                    .events
                    .push(EngineEvent::WindowCreate(win_id.clone()));
            }
        }
        Ok(None)
    }
    pub fn get_current_window(state: &mut APIMethodParams) -> APIMethodResult {
        let win_id = state.engine.active_window.clone();
        state.params = Some(json!({"win_id": win_id}));
        Self::get_window(state)
    }
    pub fn get_window(state: &mut APIMethodParams) -> APIMethodResult {
        let win_id = utils::try_parse::<WindowIdParams>(state.params.clone())?.win_id;
        let (win, doc) = state.engine.get_window(&win_id);
        Ok(Some(json!({
            "window": win,
            "document": doc
        })))
    }
    pub fn close_window(state: &mut APIMethodParams) -> APIMethodResult {
        let win_id = utils::try_parse::<WindowIdParams>(state.params.clone())?.win_id;
        if let Some(old_layout) = std::mem::take(&mut state.engine.layout) {
            let new_layout = old_layout
                .remove_window(&win_id)
                .ok_or_else(|| format!("Window `{}` not found in layout", &win_id))?;

            state.engine.layout = Some(new_layout);
        }
        state.engine.windows.remove(&win_id);

        if state.engine.active_window == win_id {
            state.engine.active_window = state
                .engine
                .windows
                .keys()
                .next()
                .cloned()
                .ok_or_else(|| "No windows left after closing window".to_string())?;
        }

        state.engine.emit(&EngineEvent::WindowClose(win_id.clone()));
        Ok(None)
    }
    pub fn move_window(state: &mut APIMethodParams) -> APIMethodResult {
        let dir = utils::try_parse::<WindowMoveParams>(state.params.clone())?.dir;
        let cur_win = state.engine.active_window.clone();
        if let Some(layout) = &state.engine.layout {
            let neighbor = layout.get_neighbor(cur_win, dir);
            if let Some(id) = neighbor {
                state.engine.active_window = id.clone();
                state.engine.emit(&EngineEvent::LayoutChange);
                return Ok(Some(json!({
                    "win_id":id
                })));
            };
            return Ok(None);
        }
        Err("No Valid Layout".to_string())
    }
    pub fn kill(state: &mut APIMethodParams) -> APIMethodResult {
        println!("quit?");
        state.engine.should_quit = true;
        Ok(None)
    }
}

impl APIRegister for EngineAPI {
    fn register_methods(api: &mut super::API) {
        let mut methods: HashMap<&str, APIMethod> = HashMap::new();
        methods.insert("window.create", Self::create_window);
        methods.insert("window.get_current", Self::get_current_window);
        methods.insert("window.get_window", Self::get_window);
        methods.insert("window.close", Self::close_window);
        methods.insert("window.move", Self::move_window);
        methods.insert("kill", Self::kill);
        api.register_api(methods);
    }
}
#[derive(Deserialize)]
struct WindowMoveParams {
    dir: SplitDirection,
}

#[derive(Deserialize)]
struct WindowIdParams {
    win_id: String,
}

#[derive(Deserialize)]
enum CreateWindowParams {
    Split {
        doc: String,
        enter: bool,
        src_win: String,
        direction: SplitDirection,
        border: Option<BorderStyle>,
        ratio: Option<f32>,
    },

    Floating {
        doc: String,
        enter: bool,
        position: PopupPosition,
        relative: RelativeTo,
        #[serde(default)]
        win: Option<String>, // if relative = win
        width: usize,
        height: usize,
        row: Option<usize>, // if position = absolute
        col: Option<usize>, // if position = absolute
        style: Option<String>,
        border: Option<BorderStyle>,
        focusable: Option<bool>,
        zindex: Option<u32>,
    },
}
