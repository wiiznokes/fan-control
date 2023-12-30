// https://github.com/crossterm-rs/crossterm/issues/397
// no blocking read timeout for now

use std::{
    sync::mpsc::{self, RecvTimeoutError, Sender},
    thread::{self},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use data::{settings::Settings, AppState};

pub fn run_cli(mut app_state: AppState) {
    let current_config = match &app_state.dir_manager.settings().current_config {
        Some(current_config) => current_config,
        None => {
            println!("There is no active configuration defined");
            println!(
                "You can define one in {} directory,",
                app_state.dir_manager.config_dir_path.display()
            );
            println!("Or specifie a config to use using a argument");
            return;
        }
    };

    let (tx, rx) = mpsc::channel::<UserAction>();
    start_listening(tx);
    display_info(app_state.dir_manager.settings(), current_config);

    loop {
        if let Err(e) = app_state.update.optimized(
            &mut app_state.app_graph.nodes,
            &app_state.app_graph.root_nodes,
            &mut app_state.bridge,
        ) {
            error!("{}", e);
        }

        let duration = Duration::from_millis(app_state.dir_manager.settings().update_delay);

        match rx.recv_timeout(duration) {
            Ok(action) => match action {
                UserAction::Quit => {
                    println!("quit requested");
                    break;
                }
            },

            Err(RecvTimeoutError::Disconnected) => {
                error!("can't listen action from user");
                break;
            }

            Err(RecvTimeoutError::Timeout) => {
                // new update cycle
            }
        }
    }

    if let Err(e) = app_state.bridge.shutdown() {
        error!("shutdown hardware: {}", e);
    }
}

enum UserAction {
    Quit,
}

#[allow(clippy::single_match)]
fn start_listening(tx: Sender<UserAction>) {
    let _handle = thread::spawn(move || loop {
        match event::read() {
            Ok(event) => match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    if let Err(e) = tx.send(UserAction::Quit) {
                        error!("can't send user action to app: {e}");
                        break;
                    }
                }
                _ => {}
            },
            Err(e) => {
                error!("can't read keyboard: {}", e);
            }
        }
    });
}

fn display_info(settings: &Settings, current_config: &String) {
    println!();
    println!("Update delay: {} ms", settings.update_delay);
    println!("Active configuration: {}", current_config);
    println!();
    println!("Available options:");
    println!("quit: q");
    println!();
}
