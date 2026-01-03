use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use serde::Serialize;
use uuid::Uuid;

use crate::{
    commands::{Key, Modifiers, command_dispatcher::Command},
    engine::{
        Edit,
        documents::{
            InsertModeProvider, spreadsheet::SpreadSheetDocumentData, text::TextDocumentData,
        },
    },
    input::keymaps::{ActionNode, KeymapProvider},
};

pub type DocId = String;

#[derive(Eq, Clone, Hash, PartialEq, Serialize)]
pub enum DocType {
    SpreadSheet,
    Info,
    Text,
}
#[derive(Serialize)]
pub struct Document {
    pub doc_type: DocType,
    pub path: Option<PathBuf>,
    pub data: DocumentData,
    #[serde(skip)]
    pub undo_stack: Vec<Edit>,
    #[serde(skip)]
    pub keymap: Option<ActionNode>,
}
impl Document {
    pub fn new(data: DocumentData, path: Option<PathBuf>) -> (DocId, Self) {
        let doc_type = match data {
            DocumentData::Text(_) => DocType::Text,
            DocumentData::SpreadSheet(_) => DocType::SpreadSheet,
            DocumentData::Help(_) => DocType::Info,
            _ => DocType::Info,
        };
        (
            Uuid::new_v4().to_string(),
            Self {
                doc_type: doc_type.clone(),
                path,
                keymap: None,
                data,
                undo_stack: vec![],
            },
        )
    }
}

#[derive(Serialize)]
pub enum DocumentData {
    SpreadSheet(SpreadSheetDocumentData),
    Text(TextDocumentData),
    Help(String),
    Config(String),
}
impl DocumentData {
    pub fn as_insertable(&mut self) -> Option<&mut dyn InsertModeProvider> {
        match self {
            Self::SpreadSheet(t) => Some(t),
            Self::Text(t) => Some(t),
            _ => None,
        }
    }
}
impl KeymapProvider for Document {
    fn get_keymap_cache(&self) -> &Option<crate::input::keymaps::ActionNode> {
        &self.keymap
    }
    fn set_keymap_cache(&mut self, value: Option<crate::input::keymaps::ActionNode>) {
        self.keymap = value;
    }
    fn define_keymap(&self) -> crate::input::keymaps::ActionNode {
        let mut keymap: HashMap<Key, ActionNode> = HashMap::new();
        keymap.insert(
            Key {
                code: crate::commands::KeyCode::Char(' '),
                modifiers: Modifiers::empty(),
            },
            ActionNode {
                children: HashMap::from([(
                    Key {
                        code: crate::commands::KeyCode::Down,
                        modifiers: Modifiers::empty(),
                    },
                    ActionNode {
                        children: HashMap::new(),
                        action: Some(crate::input::Token::Command(Command {
                            id: "window.split_scratch_down".to_string(),
                            args: vec![],
                        })),
                    },
                )]),
                action: None,
            },
        );
        ActionNode {
            children: keymap,
            action: None,
        }
    }
}
