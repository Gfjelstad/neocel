use crate::{
    commands::command_dispatcher::CommandRequest,
    engine::{Engine, document::{DocId, DocType}},
    input::{
        Token,
        keymaps::{ActionNode, Key, KeyCode, Modifiers},
    },
};
use pyo3::pyclass;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

pub struct InputEngine {
    active_nodes: Vec<Option<ActionNode>>,
    pending: PendingState,
    mode: Mode,

    pub global_map: HashMap<Mode, ActionNode>,
    pub document_map: HashMap<DocId, HashMap<Mode, ActionNode>>,
    pub doctype_map: HashMap<DocType, HashMap<Mode, ActionNode>>
}

impl InputEngine {
    pub fn new() -> Self {
        Self {
            active_nodes: vec![],
            pending: PendingState::new(),
            mode: Mode::Input,

            global_map: HashMap::new(),
            document_map: HashMap::new(),
            doctype_map: HashMap::new()
        }
    }
    pub fn feed(
        &mut self,
        key: Key,
        engine: &mut Engine,
    ) -> Result<Option<CommandRequest>, String> {
        // get potential token from key, match on token to fill out pending state, on motion or
        // command, emit command to command_dispatcher
        if let Mode::Input = self.mode {
            if !(key.code == KeyCode::Esc
                || key.modifiers.contains(Modifiers::CTRL)
                || key.modifiers.contains(Modifiers::ALT))
            {
                let (win, doc) = engine.get_current_window();
                if let Some(d) = doc.data.as_insertable() {
                    d.handle_key(win, key).unwrap();
                    return Ok(None);
                }
            } else if key.code == KeyCode::Esc {
                self.mode = Mode::Normal;
                self.reset();
            }
        }
        let token = self.get_token(engine, key);
        if token.is_none() {
            return Ok(None);
        }
        match token.unwrap() {
            Token::Digit(dig) => self.pending.count = Some(dig),
            Token::Operator(op) => self.pending.operator = Some(op),
            Token::Motion(op) => {
                self.pending.motion = Some(op);
                let cmd = self.create_operator_command();
                self.reset();
                return Ok(cmd);
            }
            Token::Command(op) => {
                self.reset();
                return Ok(Some(op));
            }
        }
        Ok(None)
    }
    fn create_operator_command(&mut self) -> Option<CommandRequest> {
        if self.pending.operator.is_none() {
            self.reset();
            return None;
        }
        let mut args = Vec::<Value>::new();
        args.push(json!({"operator": self.pending.operator}));
        if self.pending.count.is_some() {
            args.push(json!({"count":self.pending.count}));
        }
        if self.pending.motion.is_some() {
            args.push(json!({"motion":self.pending.motion}));
        }
        if self.pending.modifier.is_some() {
            args.push(json!({"modifier":self.pending.modifier}));
        }
        Some(CommandRequest {
            id: "editor.operator".to_string(),
            args,
        })
    }

    fn current_keymap_stack(&mut self, engine: &mut Engine) -> Vec<&ActionNode> {
        let (_, doc) = engine.get_current_window();
        
        let mut chain = Vec::new();
    
        // 1. Buffer-specific (highest priority)
        if let Some(buffer_modes) = self.document_map.get(&doc.id) {
            if let Some(action_node) = buffer_modes.get(&self.mode) {
                chain.push(action_node);
            }
        }
        
        // 2. Doctype-specific (middle priority)
        if let Some(doctype_modes) = self.doctype_map.get(&doc.doc_type) {
            if let Some(action_node) = doctype_modes.get(&self.mode) {
                chain.push(action_node);
            }
        }
        
        // 3. Global (lowest priority/fallback)
        if let Some(action_node) = self.global_map.get(&self.mode) {
            chain.push(action_node);
        }
        chain
    }

    pub fn mode(&self) -> Mode {self.mode.clone()}

    pub fn change_mode(&mut self, mode: Mode) {
        self.reset();
        self.mode = mode;
    }

    pub fn reset(&mut self) {
        self.active_nodes.clear();
        self.pending = PendingState::new();
    }

    fn get_token(&mut self, engine: &mut Engine, key: Key) -> Option<Token> {
        // Initialize cursors if empty
        if self.active_nodes.is_empty() {
            let keymaps = self.current_keymap_stack(engine);
            self.active_nodes = keymaps.iter().map(|&km| Some(km.clone())).collect();
        }

        let mut matches = Vec::new();


        for i in 0..self.active_nodes.len() {
            if let Some(node) = self.active_nodes.get_mut(i).unwrap() {
                if let Some(next) = node.children.get(&key) {
                    if let Some(token) = &next.action {
                        matches.push(token.clone());
                    }
                    *node = next.clone();
                } else {
                    self.active_nodes[i] = None;
                }
            }
        }

        if matches.is_empty() && self.active_nodes.iter().all(|n| n.is_none()) {
            self.reset();
            return None;
        }
        if self.active_nodes.iter().all(|n| n.is_some()) {
            return None;
        }
        if !matches.is_empty() {
            return Some(matches[0].clone());
        }
        None
    }


}

impl Default for InputEngine {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Deserialize, Clone, Debug,Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Input,
    Visualize,
    Normal,
}

pub struct PendingState {
    pub count: Option<u32>,
    pub operator: Option<String>,
    pub modifier: Option<String>,
    pub motion: Option<String>,
}
impl PendingState {
    pub fn new() -> Self {
        Self {
            count: None,
            operator: None,
            modifier: None,
            motion: None,
        }
    }
}

impl Default for PendingState {
    fn default() -> Self {
        Self::new()
    }
}


