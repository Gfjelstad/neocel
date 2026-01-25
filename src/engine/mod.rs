use std::{collections::HashMap, vec};
pub mod document;
pub mod documents;
pub mod layout;
pub mod parse;
pub mod popup;
use crate::{
    commands::{Key, command_dispatcher::CommandRequest},
    config::Config,
    engine::{
        document::{DocId, Document, DocumentData},
        documents::{DocumentDataProvider, text::TextDocumentData},
        layout::LayoutNode,
        popup::PopupWindow,
    },
    input::keymaps::{ActionNode, KeymapProvider},
    render::{Rect, helpers::BorderStyle},
};
use crossterm::event::Event;
use crossterm::terminal;
use pyo3::pyclass;
use serde::{self, Deserialize, Serialize};
use uuid::Uuid;

pub type EngineEventCallback = Box<dyn FnMut(&mut Engine, &EngineEvent)>;

pub struct Engine {
    pub events: Vec<EngineEvent>,
    pub docs: HashMap<DocId, Document>,

    pub windows: HashMap<WindowId, WindowState>,
    pub active_window: WindowId,

    pub layout: Option<LayoutNode>,
    pub popups: Option<PopupWindow>,
    pub config: Config,

    pub keymap: Option<ActionNode>,
    pub should_quit: bool,

    subscriptions: HashMap<EngineEventKind, Vec<EngineEventCallback>>,
}
impl Engine {
    pub fn new(config: Config, doc: Option<(DocId, Document)>) -> Self {
        let (doc_id, doc) = match doc {
            Some(d) => d,
            None => Document::new(DocumentData::Text(TextDocumentData::new()), None),
        };
        let (win_id, win) = WindowState::new(doc_id.clone());
        Self {
            events: vec![EngineEvent::WindowCreate(win_id.clone())],
            windows: HashMap::from([(win_id.clone(), win)]),
            popups: None,
            keymap: None,
            should_quit: false,
            active_window: win_id.clone(),
            config,
            subscriptions: HashMap::new(),
            docs: HashMap::from([(doc_id, doc)]),
            layout: Some(LayoutNode::Leaf(win_id)),
        }
    }

    pub fn subscribe(&mut self, event: EngineEventKind, func: EngineEventCallback) {
        self.subscriptions.entry(event).or_default().push(func);
    }
    pub fn emit(&mut self, event: &EngineEvent) {
        let kind = event.kind();
        let mut subs = self.subscriptions.remove(&kind).unwrap_or_default();

        for f in subs.iter_mut() {
            f(self, event);
        }
        self.subscriptions.insert(kind, subs);
        self.events.push(event.clone());
    }
    pub fn get_window(&mut self, win_id: &WindowId) -> (&mut WindowState, &mut Document) {
        let win = self.windows.get_mut(win_id).unwrap();
        let doc = self.docs.get_mut(&win.doc_id.clone()).unwrap();
        (win, doc)
    }

    pub fn get_current_window(&mut self) -> (&mut WindowState, &mut Document) {
        let win_id = self.active_window.clone();
        self.get_window(&win_id)
    }
    pub fn await_input(&mut self) -> Result<Event, String> {
        loop {
            let event = crossterm::event::read().map_err(|err| err.to_string())?;
            self.emit(&EngineEvent::InputEvent(event.clone()));
            if (event.is_mouse() && event.as_mouse_event().unwrap().kind.is_up())
                || event.is_key_press()
            {
                return Ok(event);
            }
        }
    }
    pub fn process_input(&mut self) -> Result<Option<Key>, String> {
        let event = self.await_input()?;

        match event {
            Event::Mouse(event) => {
                let (col, row) = (event.column as usize, event.row as usize);
                let (cols, rows) = terminal::size().map_err(|e| e.to_string())?;
                if let Some(layout) = &self.layout {
                    let tiles = layout.get_rects(&Rect {
                        x: 0,
                        y: 0,
                        width: cols as usize,
                        height: rows as usize,
                    });
                    for (win, rect) in tiles {
                        if rect.x <= col
                            && rect.x + rect.width > col
                            && rect.y <= row
                            && rect.y + rect.height > row
                        {
                            self.active_window = win;
                            self.emit(&EngineEvent::LayoutChange);
                        }
                    }
                }
                Ok(None)
            }
            Event::Key(key_event) => {
                let converted = crate::commands::Key::from(key_event);

                Ok(Some(converted))
            }
            _ => Ok(None),
        }

        // Normalize
    }
}
impl KeymapProvider for Engine {
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
                code: crate::commands::KeyCode::Char('q'),
                modifiers: crate::commands::Modifiers::CTRL,
            },
            ActionNode {
                children: HashMap::new(),
                action: Some(crate::input::Token::Command(CommandRequest {
                    id: "buffer.close".to_string(),
                    args: vec![],
                })),
            },
        );
        keymap.insert(
            Key {
                code: crate::commands::KeyCode::Char('c'),
                modifiers: crate::commands::Modifiers::CTRL,
            },
            ActionNode {
                children: HashMap::new(),
                action: Some(crate::input::Token::Command(CommandRequest {
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
}
#[derive(Deserialize, Clone, Copy)]
#[pyclass]
#[serde(rename_all = "snake_case")]
pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone)]
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
#[derive(Serialize, Debug)]
pub struct WindowState {
    pub id: WindowId,
    pub doc_id: DocId,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_rows: usize,
    pub scroll_cols: usize,
    pub border_style: Option<BorderStyle>,
    #[serde(skip)]
    pub keymap: Option<ActionNode>,
}
impl WindowState {
    pub fn new(doc_id: DocId) -> (WindowId, WindowState) {
        let id = Uuid::new_v4().to_string();
        (
            id.clone(),
            Self {
                id,
                doc_id,
                keymap: None,
                cursor_row: 0,
                cursor_col: 0,
                scroll_rows: 0,
                scroll_cols: 0,
                border_style: None,
            },
        )
    }
}

impl KeymapProvider for WindowState {
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
                code: crate::commands::KeyCode::Char('q'),
                modifiers: crate::commands::Modifiers::CTRL,
            },
            ActionNode {
                children: HashMap::new(),
                action: Some(crate::input::Token::Command(CommandRequest {
                    id: "buffer.close".to_string(),
                    args: vec![],
                })),
            },
        );
        ActionNode {
            children: keymap,
            action: None,
        }
    }
}
