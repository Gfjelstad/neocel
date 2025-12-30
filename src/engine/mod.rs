use std::{
    collections::{HashMap, HashSet},
    io,
    path::PathBuf,
    vec,
};
pub mod parse;
use crossterm::event::{self, Event, KeyEvent};
use uuid::Uuid;

use crate::{config::Config, engine::parse::parse_csv_to_doc};

pub struct Engine {
    pub events: Vec<EngineEvent>,
    pub command_context: Vec<u8>,
    pub docs: HashMap<DocId, Document>,

    pub windows: HashMap<WindowId, WindowState>,
    pub active_window: WindowId,

    pub layout: LayoutNode,
    pub config: Config,
}
impl Engine {
    pub fn new(config: Config) -> Self {
        // let doc = parse_csv_to_doc(sheet).expect("Failed To Parse CSV");
        let doc_id = Uuid::new_v4().to_string();
        let window_id = Uuid::new_v4().to_string();
        let engine = Self {
            events: vec![EngineEvent::WindowCreate(window_id.clone())],
            windows: HashMap::from([(
                window_id.clone(),
                WindowState {
                    doc_id: doc_id.clone(),
                    cursor_row: 0,
                    cursor_col: 0,
                    scroll_rows: 0,
                    scroll_cols: 0,
                },
            )]),
            active_window: window_id.clone(),
            config: config,
            docs: HashMap::from([(
                doc_id.clone(),
                Document {
                    doc_type: DocType::Text,
                    path: None,
                    undo_stack: vec![],
                    data: DocumentData::Text("".to_string()),
                },
            )]),
            command_context: vec![],
            layout: LayoutNode::Leaf(window_id),
        };

        return engine;
    }

    pub fn await_input(&mut self) -> Result<KeyEvent, String> {
        loop {
            if let Event::Key(event) = crossterm::event::read().map_err(|err| err.to_string())? {
                return Ok(event);
            }
        }
    }

    pub fn replace_window_document(
        &mut self,
        doc: Document,
        window_id: WindowId,
        split: Option<bool>,
    ) -> WindowId {
        let doc_id = Uuid::new_v4().to_string();
        self.docs.insert(doc_id.clone(), doc);
        let window_id = Uuid::new_v4().to_string();
        self.windows.insert(
            window_id.clone(),
            WindowState {
                doc_id: doc_id.clone(),
                cursor_row: 0,
                cursor_col: 0,
                scroll_rows: 0,
                scroll_cols: 0,
            },
        );
        self.events.push(EngineEvent::WindowDocChange(
            window_id.clone(),
            doc_id.clone(),
        ));
        if let Some(mut node) = self.layout.find_child(window_id.clone()) {
            node = &mut LayoutNode::Leaf(window_id.clone());
        }

        return window_id;
    }
}
pub enum EngineEvent {
    WindowCreate(WindowId),
    WindowClose(WindowId),
    WindowDocChange(WindowId, DocId),
    LayoutChange,
    DocumentCreate(DocId),
}
pub type DocId = String;
pub enum DocType {
    SpreadSheet,
    Info,
    Text,
}
pub struct Document {
    pub doc_type: DocType,
    pub path: Option<PathBuf>,
    pub data: DocumentData,
    pub undo_stack: Vec<Edit>,
}
pub enum DocumentData {
    SpreadSheet(HashMap<usize, HashMap<usize, Cell>>),
    Text(String),
    Help(String),
    Config(String),
}
pub struct Edit {}
pub type WindowId = String;
pub struct WindowState {
    pub doc_id: DocId,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_rows: usize,
    pub scroll_cols: usize,
}
pub enum SplitDir {
    Vert,
    Horz,
}
pub enum LayoutNode {
    Leaf(WindowId),
    Split {
        direction: SplitDir,
        ratio: f32,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
    },
}
impl LayoutNode {
    fn find_child(&mut self, window_id: String) -> Option<&mut LayoutNode> {
        let mut search = vec![self];

        while search.len() > 0 {
            let current = search.pop().expect("current LayoutNode does not exist");
            match current {
                LayoutNode::Leaf(curr_id) => {
                    if &window_id == curr_id {
                        return Some(current);
                    }
                }
                LayoutNode::Split {
                    direction,
                    ratio,
                    first,
                    second,
                } => {
                    search.push(first);
                    search.push(second);
                }
            }
        }
        None
    }
}
pub type CellId = String;
pub struct Cell {
    pub raw: String,
    pub value: CellValue,
    pub ast: Option<Expr>,
    pub dependencies: HashSet<CellId>,
    pub used_by: HashSet<CellId>,
}
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Error(String),
}

impl CellValue {
    pub fn from_str(s: &str) -> Self {
        let trimmed = s.trim();

        // Check if empty
        if trimmed.is_empty() {
            return CellValue::Empty;
        }

        // Try to parse as number
        match trimmed.parse::<f64>() {
            Ok(num) => CellValue::Number(num),
            Err(_) => CellValue::Text(trimmed.to_string()),
        }
    }
}

pub struct Expr {}
