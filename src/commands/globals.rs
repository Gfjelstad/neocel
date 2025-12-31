use crate::engine::{self, Engine, EngineEvent, document::DocumentData};

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
    let data = crate::engine::document::DocumentData::Text(vec!["".to_string()]);
    engine.split_window_document(data, crate::engine::SplitDirection::Down);
    Ok(())
}

pub fn hello_world_popup(engine: &mut Engine) -> Result<(), String> {
    let data = crate::engine::document::DocumentData::Text(vec![
        String::new(),
        String::new(),
        String::new(),
    ]);
    let (win_id, doc_id) =
        engine.create_popup(data, 40, 7, crate::engine::PopupPosition::TopRight)?;
    engine.subscribe(
        crate::engine::EngineEventKind::InputEvent,
        Box::new(move |engine, event| {
            if let EngineEvent::InputEvent(input) = event {
                let win = &engine.windows[&win_id];
                let doc = engine.docs.get_mut(&doc_id).unwrap();
                if let DocumentData::Text(data) = &mut doc.data {
                    if let Some(mouse) = input.as_mouse_event() {
                        if mouse.kind.is_up() {
                            data[0] = format!("Mouse Position: {:?},{:?}", mouse.row, mouse.column);
                        }
                    } else {
                        data[1] =
                            format!("Cursor Position: {:?},{:?}", win.cursor_row, win.cursor_col);
                    }
                }
            }
        }),
    );
    Ok(())
}
