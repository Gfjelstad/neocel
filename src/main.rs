use std::{
    collections::HashMap,
    env,
    ffi::CString,
    fs::File,
    io::{Write, stdout},
    path::PathBuf,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use log::LevelFilter;
use pyo3::{
    Py, Python,
    types::{PyAnyMethods, PyModuleMethods},
};
use simplelog::WriteLogger;
pub mod api;
pub mod commands;
pub mod config;
pub mod engine;
pub mod input;
pub mod render;

use crate::{
    commands::command_dispatcher::{
        ApiContext, CommandDispatcher, CommandFunction, CommandRequest,
    },
    config::Config,
    engine::{Engine, parse::parse_csv_to_doc},
    input::input_engine::InputEngine,
    render::UI,
};
fn main() -> Result<(), String> {
    init_logger();
    let mut _args: Vec<String> = env::args().collect();
    if _args.len() == 1 {
        _args.push(String::new())
    }
    _args[1] = "./test.csv".to_string();
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();
    stdout().execute(EnableMouseCapture).unwrap();

    let res = main_loop(_args);

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
        .queue(cursor::MoveTo(0, 0))
        .unwrap()
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
    ui.handle_events(&mut engine);
    ui.draw(&mut engine, &input_engine);
    log::info!("Successfully created engines");
    _ =command_dispatcher.dispatch(
        &CommandRequest {
            id: "init".to_string(),
            args: vec![],
        },
        &mut engine,
        &mut input_engine,
        &mut ui,
    );
    // initial commands before awaiting an input;
    loop {
        if let Some(key) = engine.process_input()?
            && let Some(cmd) = input_engine.feed(key, &mut engine)?
        {
            let res = command_dispatcher.dispatch(&cmd, &mut engine, &mut input_engine, &mut ui);
            match res {
                Ok(_) => log::info!("OK running command {:?}", cmd.id),
                Err(err) => log::warn!("failed running command {:?}: {:?}", cmd.id, err),
            }
        }
        if engine.should_quit {
            break;
        }
        ui.handle_events(&mut engine);
        ui.draw(&mut engine, &input_engine);
    }
    Ok(())
}

fn setup_input_engine(_config: &Config) -> InputEngine {
    InputEngine::new()
}
fn setup_command_dispatcher(_config: &Config) -> CommandDispatcher {
    let mut cmd_disp = CommandDispatcher::new();

    cmd_disp.register_global("kill", CommandFunction::Internal("kill".to_string(), None));
    cmd_disp.register_global(
        "init",
        CommandFunction::Rust(Box::new(|api, params| {
            Python::attach(|py| {
                // Create the API object
                let api = api.to_py_api()?;

                // Get the __main__ module's globals
                let main_module = py
                    .import("__main__")
                    .map_err(|e| format!("Failed to import __main__: {}", e))?;

                let globals = main_module.dict();

                // Add the API to globals as 'api'
                globals
                    .set_item("api", api)
                    .map_err(|e| format!("Failed to set api in globals: {}", e))?;
                let init_path = "./test/init.py";
                // Read the init.py file
                let code = std::fs::read_to_string(init_path)
                    .map_err(|e| format!("Failed to read {}: {}", init_path, e))?;
                let code_cstr = CString::new(code)
                    .map_err(|e| format!("Failed to convert code to CString: {}", e))?;
                // Execute the init.py file with the globals containing 'api'
                py.run(code_cstr.as_c_str(), Some(&globals), None)
                    .map_err(|e| format!("Failed to execute {}: {}", init_path, e))?;

                Ok(None)
            })
        })),
    );
    cmd_disp
}

fn setup_config() -> config::Config {
    let mut config = config::Config {
        init_location: None,
        settings: HashMap::new(),
        keybinds: HashMap::new(),
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
}
fn setup_engine(config: Config, args: Vec<String>) -> Engine {
    let mut doc = None;
    if !args.is_empty() && args[1].contains(".csv") {
        doc = Some(
            parse_csv_to_doc(PathBuf::from(args[1].clone()))
                .map_err(|e| println!("{:?}", e.to_string()))
                .unwrap(),
        );
    }
    Engine::new(config, doc)
}
fn setup_ui(config: &Config) -> UI {
    UI::new(config)
}
fn init_logger() {
    let mut log_path = env::current_exe()
        .expect("Failed to get exe path")
        .parent()
        .expect("EXE must live in a directory")
        .to_path_buf();
    log_path.push("neocel.log");
    let file = File::create(log_path).expect("Failed to create log file");

    WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), file)
        .expect("Failed to initialize logger");
}
