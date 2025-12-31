use std::{
    collections::HashMap,
    env,
    io::{Write, stdout},
    panic,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

use crate::{
    commands::globals,
    config::{Config, parse_keymap},
    engine::Engine,
    render::UI,
};
pub mod commands;
pub mod config;
pub mod engine;
pub mod render;
fn main() -> Result<(), String> {
    let _args: Vec<String> = env::args().collect();
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();
    stdout().execute(EnableMouseCapture).unwrap();

    // let res = panic::catch_unwind(|| {
    let _ = main_loop(_args);
    // });

    stdout().execute(DisableMouseCapture).unwrap();
    stdout().execute(Show).unwrap();
    disable_raw_mode().map_err(|e| e.to_string())?;

    stdout()
        .queue(Clear(ClearType::All))
        .unwrap()
        // 2. Move the cursor to the top-left position (column 0, row 0)
        .queue(cursor::MoveTo(0, 0))
        .unwrap()
        // 3. Flush the commands to the terminal
        .flush()
        .unwrap();

    Ok(())
}

fn main_loop(_args: Vec<String>) -> Result<(), String> {
    let config = setup_config();
    let mut ui = setup_ui(&config);
    let mut engine = setup_engine(config);
    ui.handle_events(&mut engine);
    ui.draw(&mut engine);
    loop {
        let ran = engine.process_input()?;
        if engine.should_quit {
            break;
        }
        if let Some(val) = ran {
            let window_id = engine.active_window.clone(); // copy or clone first

            let window = ui.windows.get_mut(&window_id).unwrap();
            window.handle_key(val, &mut engine);
        }

        ui.handle_events(&mut engine);
        ui.draw(&mut engine);
    }
    Ok(())
}

fn setup_config() -> config::Config {
    let mut config = config::Config {
        settings: HashMap::new(),
        keybinds: parse_keymap(&HashMap::from([
            ("C-f".to_string(), "kill".to_string()),
            ("C-S-down".to_string(), "split-scratch-down".to_string()),
            ("C-h".to_string(), "hello-world".to_string()),
            ("C-q".to_string(), "close-window".to_string()),
        ])),
        styles: HashMap::new(),
        commands: HashMap::new(),
    };
    config
        .styles
        .insert("background".to_string(), "#1D1D1D".to_string());
    config
        .styles
        .insert("foreground".to_string(), "#F54927".to_string());

    config
        .commands
        .insert("move_down".to_string(), globals::move_down);
    config.commands.insert("kill".to_string(), globals::kill);
    config
        .commands
        .insert("close-window".to_string(), globals::close_window);
    config
        .commands
        .insert("hello-world".to_string(), globals::hello_world_popup);
    config.commands.insert(
        "split-scratch-down".to_string(),
        globals::split_scratch_down,
    );

    config
}
fn setup_engine(config: Config) -> Engine {
    Engine::new(config)
}
fn setup_ui(config: &Config) -> UI {
    UI::new(config)
}
