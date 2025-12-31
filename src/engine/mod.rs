use std::{
    collections::{HashMap, HashSet},
    io,
    path::PathBuf,
    str::FromStr,
    vec,
};
pub mod document;
pub mod layout;
pub mod parse;
pub mod popup;
use crossterm::event::{self, Event, KeyEvent, KeyEventState};
use uuid::Uuid;

use crate::{
    config::{CommandFn, Config},
    engine::{
        document::{DocId, DocType, Document, DocumentData},
        layout::{LayoutNode, SplitDir},
        parse::parse_csv_to_doc,
        popup::{PopupPosition, PopupWindow},
    },
    render::Rect,
};

pub type EngineEventCallback = Box<dyn FnMut(&mut Engine, &EngineEvent)>;

pub struct Engine {
    pub events: Vec<EngineEvent>,
    pub command_context: Vec<u8>,
    pub docs: HashMap<DocId, Document>,

    pub windows: HashMap<WindowId, WindowState>,
    pub active_window: WindowId,

    pub layout: LayoutNode,
    pub popups: Option<PopupWindow>,
    pub config: Config,

    pub should_quit: bool,

    subscriptions: HashMap<EngineEventKind, Vec<EngineEventCallback>>,
}
impl Engine {
    pub fn new(config: Config) -> Self {
        // let doc = parse_csv_to_doc(sheet).expect("Failed To Parse CSV");
        let doc_id = Uuid::new_v4().to_string();
        let window_id = Uuid::new_v4().to_string();
        Self {
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
            popups: None,
            should_quit: false,
            active_window: window_id.clone(),
            config,
            subscriptions: HashMap::new(),
            docs: HashMap::from([(
                doc_id.clone(),
                Document {
                    doc_type: DocType::Text,
                    path: None,
                    undo_stack: vec![],
                    data: DocumentData::Text(vec!["".to_string()]),
                },
            )]),
            command_context: vec![],
            layout: LayoutNode::Leaf(window_id),
        }
    }
    pub fn subscribe(&mut self, event: EngineEventKind, func: EngineEventCallback) {
        self.subscriptions
            .entry(event)
            .or_insert_with(Vec::new)
            .push(func);
    }
    pub fn emit(&mut self, event: &EngineEvent) {
        let kind = event.kind();
        let mut subs = self.subscriptions.remove(&kind).unwrap_or_default();

        for f in subs.iter_mut() {
            f(self, event);
        }
        self.subscriptions.insert(kind, subs);
    }

    pub fn get_current_window(&mut self) -> (&mut WindowState, &mut Document) {
        let win_id = self.active_window.clone();
        let win = self.windows.get_mut(&win_id).unwrap();
        let doc = self.docs.get_mut(&win.doc_id.clone()).unwrap();
        (win, doc)
    }
    pub fn await_input(&mut self) -> Result<KeyEvent, String> {
        loop {
            let event = crossterm::event::read().map_err(|err| err.to_string())?;
            self.emit(&EngineEvent::InputEvent(event.clone()));
            if let Event::Key(event) = event {
                return Ok(event);
            }
        }
    }
    pub fn process_input(&mut self) -> Result<Option<KeyEvent>, String> {
        let key_event = self.await_input()?;

        // Normalize
        let normalized = KeyEvent {
            code: key_event.code,
            modifiers: key_event.modifiers,
            state: KeyEventState::empty(),
            kind: event::KeyEventKind::Press,
        };
        if key_event.kind != event::KeyEventKind::Press {
            return Ok(Some(key_event));
        }

        let command_name = self.config.keybinds.get(&normalized).cloned();

        let command_fn = command_name.and_then(|name| self.config.commands.get(&name).cloned());

        if let Some(fun) = command_fn {
            fun(self)?;
            return Ok(None);
        }
        Ok(Some(key_event))
    }
    pub fn create_popup(
        &mut self,
        doc: DocumentData,
        width: usize,
        height: usize,
        pos: PopupPosition,
    ) -> Result<(WindowId, DocId), String> {
        if !(matches!(doc, DocumentData::Text(_)) || matches!(doc, DocumentData::Help(_))) {
            return Err("Document Data must be either Text or Help".to_string());
        }
        let (doc_id, doc) = Document::new(doc, None);
        let (win_id, win) = WindowState::new(doc_id.clone());
        self.windows.insert(win_id.clone(), win);
        self.docs.insert(doc_id.clone(), doc);

        self.popups = Some(PopupWindow {
            position: pos,
            width,
            height,
            layout: LayoutNode::Leaf(win_id.clone()),
        });
        self.events.push(EngineEvent::WindowCreate(win_id.clone()));
        Ok((win_id, doc_id))
    }
    pub fn close_window(&mut self, window_id: &String) -> Result<(), String> {
        self.windows.remove(window_id);
        Ok(())
    }
    pub fn close_doc(&mut self, doc_id: &String) -> Result<(), String> {
        self.docs.remove(doc_id);
        Ok(())
    }

    pub fn split_window_document(&mut self, doc: DocumentData, direction: SplitDirection) {
        let (doc_id, doc) = Document::new(doc, None);
        let window_id = Uuid::new_v4().to_string();
        self.docs.insert(doc_id.clone(), doc);
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
        if let Some(old_node) = self.layout.find_child(self.active_window.clone()) {
            let cloned: LayoutNode = old_node.clone();
            let new_node = LayoutNode::Leaf(window_id.clone());
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
                ratio: 0.5,
                first: Box::new(first),
                second: Box::new(second),
            };
            self.events
                .push(EngineEvent::WindowCreate(window_id.clone()));
        }
    }
}

pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}
pub enum EngineEvent {
    WindowCreate(WindowId),
    WindowClose(WindowId),
    WindowDocChange(WindowId, DocId),
    LayoutChange,
    DocumentCreate(DocId),
    InputEvent(Event),
}

impl EngineEvent {
    pub fn kind(&self) -> EngineEventKind {
        match self {
            EngineEvent::WindowCreate(_) => EngineEventKind::WindowCreate,
            EngineEvent::WindowClose(_) => EngineEventKind::WindowClose,
            EngineEvent::WindowDocChange(_, _) => EngineEventKind::WindowDocChange,
            EngineEvent::LayoutChange => EngineEventKind::LayoutChange,
            EngineEvent::DocumentCreate(_) => EngineEventKind::DocumentCreate,
            EngineEvent::InputEvent(_) => EngineEventKind::InputEvent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineEventKind {
    WindowCreate,
    WindowClose,
    WindowDocChange,
    LayoutChange,
    DocumentCreate,
    InputEvent,
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
impl WindowState {
    pub fn new(doc_id: DocId) -> (WindowId, WindowState) {
        (
            Uuid::new_v4().to_string(),
            Self {
                doc_id,
                cursor_row: 0,
                cursor_col: 0,
                scroll_rows: 0,
                scroll_cols: 0,
            },
        )
    }
}
