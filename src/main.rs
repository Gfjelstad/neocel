use std::{
    collections::HashMap,
    env,
    io::{Write, stdout},
    panic,
    path::PathBuf,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use pyo3::Python;
pub mod api;
pub mod commands;
pub mod config;
pub mod engine;
pub mod input;
pub mod render;

use crate::{
    api::API,
    commands::{
        CommandRegistry,
        command_dispatcher::{CommandContext, CommandDispatcher},
        globals::{self},
    },
    config::{Config, parse_keymap},
    engine::{Engine, parse::parse_csv_to_doc},
    input::input_engine::InputEngine,
    render::UI,
};
fn main() -> Result<(), String> {
    let mut _args: Vec<String> = env::args().collect();
    if _args.len() == 1 {
        _args.push(String::new())
    }
    _args[1] = "./test.csv".to_string();
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();
    stdout().execute(EnableMouseCapture).unwrap();

    // let res = panic::catch_unwind(|| {
    let res = main_loop(_args);
    // });

    stdout().execute(DisableMouseCapture).unwrap();
    stdout().execute(Show).unwrap();
    disable_raw_mode().map_err(|e| e.to_string())?;

    if res.is_err() {
        println!("{:?}", res.err().unwrap().to_string());
        return Ok(());
    }
    stdout()
        .queue(Clear(ClearType::All))
        .unwrap()
        // 2. Move the cursor to the top-left position (column 0, row 0)
        .queue(cursor::MoveTo(0, 0))
        .unwrap()
        // 3. Flush the commands to the terminal
        .flush()
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn main_loop(args: Vec<String>) -> Result<(), String> {
    let config = setup_config();
    let mut ui = setup_ui(&config);
    let mut input_engine = setup_input_engine(&config);
    let mut command_dispatcher = setup_command_dispatcher(&config);
    let mut engine = setup_engine(config, args);
    let mut api = API::new();
    ui.handle_events(&mut engine);
    ui.draw(&mut engine);
    loop {
        let ran = engine.process_input()?;
        if let Some(key) = ran
            && let Some(cmd) = input_engine.feed(key, &mut engine)?
        {
            api.runcommand(
                &mut engine,
                &mut input_engine,
                &mut ui,
                &mut command_dispatcher,
                |api| {
                    let res = command_dispatcher.dispatch(
                        &engine::document::DocType::SpreadSheet,
                        CommandContext { api: api },
                        &cmd,
                    );
                },
            );
        }

        if engine.should_quit {
            break;
        }
        ui.handle_events(&mut engine);
        ui.draw(&mut engine);
    }
    Ok(())
}
fn setup_input_engine(_config: &Config) -> InputEngine {
    let ie = InputEngine::new();
    ie
}
fn setup_command_dispatcher(_config: &Config) -> CommandDispatcher {
    let cmd_disp = CommandDispatcher::new();

    // DefaultGlobalCommands::register_commands(&mut cmd_disp).unwrap();
    cmd_disp
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
        .insert("background_secondary".to_string(), "#353535".to_string());
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
fn setup_engine(config: Config, args: Vec<String>) -> Engine {
    let mut doc = None;
    if args.len() >= 1 && args[1].contains(".csv") {
        doc = Some(
            parse_csv_to_doc(PathBuf::from(args[1].clone()))
                .map_err(|e| println!("{:?}", e.to_string()))
                .unwrap(),
        );
    }
    let e = Engine::new(config, doc);

    e
}
fn setup_ui(config: &Config) -> UI {
    UI::new(config)
}
