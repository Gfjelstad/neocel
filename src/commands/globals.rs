use serde_json::{Value, json};

use crate::{
    commands::CommandRegistry,
    engine::{
        Engine, EngineEvent,
        document::DocumentData,
        documents::{DocumentDataProvider, text::TextDocumentData},
        popup::PopupPosition,
    },
};

pub fn move_down(engine: &mut Engine) -> Result<(), String> {
    if let Some(win) = engine.windows.get_mut(&engine.active_window) {
        win.cursor_row += 1;
    }
    Ok(())
}

pub fn kill(engine: &mut Engine) -> Result<(), String> {
    // engine.should_quit = true; // or however you exit
    engine.should_quit = true;
    Ok(())
}

pub fn split_scratch_down(engine: &mut Engine) -> Result<(), String> {
    let data = crate::engine::document::DocumentData::Text(TextDocumentData::new(""));
    engine.split_window_document(data, crate::engine::SplitDirection::Down);
    Ok(())
}

pub fn close_window(engine: &mut Engine) -> Result<(), String> {
    let doc_id = engine.windows[&engine.active_window.clone()].doc_id.clone();
    engine.close_doc(&doc_id)?;
    engine.close_window(&engine.active_window.clone())?;
    Ok(())
}

pub fn hello_world_popup(engine: &mut Engine) -> Result<(), String> {
    let data = crate::engine::document::DocumentData::Text(TextDocumentData::new("\n\n"));
    let (win_id, doc_id) = engine.create_popup(data, 40, 7, PopupPosition::TopRight)?;
    engine.subscribe(
        crate::engine::EngineEventKind::InputEvent,
        Box::new(move |engine, event| {
            if let EngineEvent::InputEvent(input) = event {
                let _win = &engine.windows[&win_id];
                let doc = engine.docs.get_mut(&doc_id).unwrap();
                let current_window = &engine.windows[&engine.active_window.clone()];
                if let DocumentData::Text(data) = &mut doc.data {
                    if let Some(mouse) = input.as_mouse_event() {
                        if mouse.kind.is_up() {
                            data.data[0] =
                                format!("Mouse Position: {:?},{:?}", mouse.row, mouse.column);
                        }
                    } else {
                        data.data[1] = format!(
                            "Cursor Position: {:?},{:?}",
                            current_window.cursor_row, current_window.cursor_col
                        );
                    }
                }
            }
        }),
    );
    Ok(())
}

pub struct DefaultGlobalCommands {}

impl DefaultGlobalCommands {}
impl CommandRegistry for DefaultGlobalCommands {
    fn register_commands(
        dispatcher: &mut super::command_dispatcher::CommandDispatcher,
    ) -> Result<(), String> {
        dispatcher.register_global(
            "window.hello_world",
            crate::commands::command_dispatcher::CommandFunction::Rust(Box::new(|ctx, _args| {
                hello_world_popup(ctx.engine)?;
                Ok(json!({}))
            })),
        );
        dispatcher.register_global(
            "window.close_current",
            crate::commands::command_dispatcher::CommandFunction::Rust(Box::new(|ctx, _args| {
                close_window(ctx.engine)?;
                Ok(json!({}))
            })),
        );
        dispatcher.register_global(
            "window.split_scratch_down",
            crate::commands::command_dispatcher::CommandFunction::Rust(Box::new(|ctx, _args| {
                split_scratch_down(ctx.engine)?;
                Ok(json!({}))
            })),
        );
        dispatcher.register_global(
            "kill",
            crate::commands::command_dispatcher::CommandFunction::Rust(Box::new(|ctx, _args| {
                kill(ctx.engine)?;
                Ok(json!({}))
            })),
        );
        Ok(())
    }
}
