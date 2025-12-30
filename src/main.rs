use std::{collections::HashMap, env, fs::File};

use crossterm::{event::KeyCode, terminal};

use crate::{config::Config, engine::Engine, render::UI};

pub mod config;
pub mod engine;
pub mod render;
fn main() {
    let args: Vec<String> = env::args().collect();
    //

    // if (args.len() == 1) {
    //     return;
    // }

    let mut config = setup_config();
    let mut engine = setup_engine(config);
    let mut ui = setup_ui();
    loop {
        ui.handle_events(&mut engine);

        println!("waiting for input");
        match engine.await_input() {
            Ok(event) => {
                if event.code == KeyCode::Char('F') {
                    panic!("success");
                }
                println!("{:?}\n", event.code.to_string())
            }
            Err(str) => {
                panic!("failed");
            }
        }
    }
    println!("out of loop")
}

fn setup_config() -> config::Config {
    let mut config = config::Config {
        settings: HashMap::new(),
        keybinds: HashMap::new(),
        styles: HashMap::new(),
        commands: HashMap::new(),
    };
    config
        .styles
        .insert("background".to_string(), "#FFFFFF".to_string());
    config
        .styles
        .insert("foreground".to_string(), "#000000".to_string());

    config.commands.insert(
        "move_down".to_string(),
        Box::new(|engine: &mut Engine| {
            engine
                .windows
                .get_mut(&engine.active_window)
                .unwrap()
                .cursor_row += 1
        }),
    );
    config.commands.insert(
        "kill".to_string(),
        Box::new(|_: &mut Engine| {
            panic!();
        }),
    );
    return config;
}
fn setup_engine(config: Config) -> Engine {
    let (cols, rows) = terminal::size().expect("could not access terminal size");
    return Engine::new(config);
}
fn setup_ui() -> UI {
    UI {
        windows: HashMap::new(),
    }
}
