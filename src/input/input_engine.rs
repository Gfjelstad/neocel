use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{Value, json};

use crate::{
    commands::{Key, KeyCode, Modifiers, command_dispatcher::Command},
    engine::{Engine, document::DocumentData, documents::InsertModeProvider},
    input::{
        Token,
        keymaps::{ActionNode, KeymapProvider},
    },
};

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
pub struct InputEngine {
    active_nodes: Vec<Option<ActionNode>>,
    pending: PendingState,
    pub mode: Mode,
}
#[derive(Deserialize, Clone, Debug)]
pub enum ModeType {
    Input,
    Visualize,
    Normal,
}
pub struct Mode {
    mode: ModeType,
    keymap: Option<ActionNode>,
}
impl Mode {
    pub fn new(mode: ModeType) -> Self {
        Self {
            mode: mode.clone(),
            keymap: None,
        }
    }
}
impl KeymapProvider for Mode {
    fn define_keymap(&self) -> ActionNode {
        let mut keymap: HashMap<Key, ActionNode> = HashMap::new();
        keymap.insert(
            Key {
                code: crate::commands::KeyCode::Char('f'),
                modifiers: crate::commands::Modifiers::CTRL,
            },
            ActionNode {
                children: HashMap::new(),
                action: Some(crate::input::Token::Command(Command {
                    id: "kill".to_string(),
                    args: vec![],
                })),
            },
        );
        ActionNode {
            children: keymap,
            action: None,
        }
    }
    fn get_keymap_cache(&self) -> &Option<ActionNode> {
        &self.keymap
    }
    fn set_keymap_cache(&mut self, value: Option<ActionNode>) {
        self.keymap = value;
    }
}
impl InputEngine {
    pub fn new() -> Self {
        Self {
            active_nodes: vec![],
            pending: PendingState::new(),
            mode: Mode::new(ModeType::Input),
        }
    }
    pub fn feed(&mut self, key: Key, engine: &mut Engine) -> Result<Option<Command>, String> {
        // get potential token from key, match on token to fill out pending state, on motion or
        // command, emit command to command_dispatcher
        if let ModeType::Input = self.mode.mode {
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
                self.mode.mode = ModeType::Normal;
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
    fn create_operator_command(&mut self) -> Option<Command> {
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
        Some(Command {
            id: "editor.operator".to_string(),
            args,
        })
    }

    fn current_keymap_stack(&mut self, engine: &mut Engine) -> Vec<ActionNode> {
        let (win, doc) = engine.get_current_window();
        vec![
            doc.keymap().clone(),
            win.keymap().clone(),
            self.mode.keymap().clone(), // <- mode layer
            engine.keymap().clone(),
        ]
    }

    fn reset(&mut self) {
        self.active_nodes.clear();
        self.pending = PendingState::new();
    }

    fn get_token(&mut self, engine: &mut Engine, key: Key) -> Option<Token> {
        // Initialize cursors if empty
        if self.active_nodes.is_empty() {
            let keymaps = self.current_keymap_stack(engine);
            self.active_nodes = keymaps.iter().map(|km| Some(km.clone())).collect();
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
