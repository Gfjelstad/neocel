use std::{
    collections::HashMap,
    env,
    ffi::CString,
    fs::File,
    io::{Write, stdout},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
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
    types::{PyAnyMethods, PyList, PyListMethods, PyModuleMethods},
};
use serde_json::json;
use simplelog::WriteLogger;
pub mod api;
pub mod commands;
pub mod config;
pub mod engine;
pub mod input;
pub mod render;
use crate::{
    commands::command_dispatcher::{CommandDispatcher, CommandFunction, CommandRequest},
    config::{Config, Theme},
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
    let input_engine = Arc::new(Mutex::new(setup_input_engine(&config)));
    let command_dispatcher = Arc::new(Mutex::new(setup_command_dispatcher(&config)));
    let engine = Arc::new(Mutex::new(setup_engine(config, args)));

    _ = CommandDispatcher::dispatch(
        &CommandRequest {
            id: "init".to_string(),
            args: vec![],
        },
        command_dispatcher.clone(),
        engine.clone(),
        input_engine.clone(),
    )
    .map_err(|e| log::error!("Failed to initialize: {}", e));

    let mut ui = setup_ui(&engine.lock().unwrap().config);
    {
        let mut_engine = &mut *engine.lock().unwrap();
        let mut_input_engine = &mut *input_engine.lock().unwrap();
        ui.handle_events(mut_engine);
        ui.draw(mut_engine, mut_input_engine);
        log::info!("Successfully created engines");
    }

    loop {
        let mut engine_guard = engine.lock().unwrap();
        let mut input_engine_guard = input_engine.lock().unwrap();

        let command = if let Some(key) = engine_guard.process_input()? {
            input_engine_guard.feed(key, &mut *engine_guard)?
        } else {
            None
        };
        drop(engine_guard);
        drop(input_engine_guard);

        if let Some(cmd) = command {
            _ = CommandDispatcher::dispatch(
                &cmd,
                command_dispatcher.clone(),
                engine.clone(),
                input_engine.clone(),
            )
            .map_err(|e| log::error!("Failed to run command: {} | {}", cmd.id.clone(), e));
        }

        let mut engine_guard = engine.lock().unwrap();
        if engine_guard.should_quit {
            break;
        }
        ui.handle_events(&mut *engine_guard);
        let mut input_engine_guard = input_engine.lock().unwrap();
        ui.draw(&mut *engine_guard, &mut *input_engine_guard);
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
                // let api = api.to_py_api()?;
                let module = api.to_module(py).map_err(|e| e.to_string())?;

                // Get the __main__ module's globals
                let main_module = py
                    .import("__main__")
                    .map_err(|e| format!("Failed to import __main__: {}", e))?;

                let globals = main_module.dict();

                // Add the API to globals as 'api'
                // globals
                //     .set_item("api", api)
                let sys = py.import("sys").map_err(|e| e.to_string())?;
                //     .map_err(|e| format!("Failed to set api in globals: {}", e))?;
               sys
                    .getattr("modules")
                    .map_err(|e| e.to_string())?
                    .set_item("api", module.clone())
                    .map_err(|e| e.to_string())?;

                let init_path = "./test/init.py";
                let script_dir = Path::new(init_path)
                    .parent()
                    .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid script path")).map_err(|e|e.to_string())?
                    .to_str()
                    .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Non-UTF8 path")).map_err(|e|e.to_string())?;


                sys.getattr("path").map_err(|e| e.to_string())?.cast::<PyList>().map_err(|e| e.to_string())?.insert(0, script_dir).map_err(|e| e.to_string())?;

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
        theme: Theme::try_from(json!({
            "background":"#000000",
            "foreground":"#FFFFFF"
        }))
        .expect("could not parse default theme"),
    };

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
