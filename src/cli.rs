// https://github.com/crossterm-rs/crossterm/issues/397
// no blocking read timeout for now

use std::{
    sync::mpsc::{self, Sender},
    thread,
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
        app_state.update.optimized(
            &mut app_state.app_graph.nodes,
            &app_state.app_graph.root_nodes,
            &mut app_state.bridge,
        );

        let duration = Duration::from_millis(app_state.dir_manager.settings().update_delay);

        if let Ok(action) = rx.recv_timeout(duration) {
            match action {
                UserAction::Quit => {
                    println!("quit requested");
                    break;
                }
            }
        }
    }
    
    if let Err(e) = app_state.bridge.shutdown() {
        error!("{:?}", e);
    }
}

enum UserAction {
    Quit,
}

#[allow(clippy::single_match)]
fn start_listening(tx: Sender<UserAction>) {
    let _join = thread::spawn(move || loop {
        match event::read() {
            Ok(event) => match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => tx.send(UserAction::Quit).unwrap(),
                _ => {}
            },
            Err(e) => {
                eprintln!("{:?}", e);
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
