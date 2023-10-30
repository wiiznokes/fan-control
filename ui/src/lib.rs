#![allow(dead_code)]

use std::time::Duration;

use data::AppState;
use iced::{self, executor, time, widget::Text, Application, Command};

pub fn run_ui(app_state: AppState) -> Result<(), iced::Error> {
    let settings = iced::Settings::with_flags(app_state);

    Ui::run(settings)
}
pub struct Ui {
    app_state: AppState,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Tick,
}

impl Application for Ui {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = AppState;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ui_state = Ui { app_state: flags };

        dbg!(&ui_state.app_state.app_graph);

        (ui_state, Command::none())
    }

    fn title(&self) -> String {
        String::from("fan-control-rs")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            AppMsg::Tick => {
                match self.app_state.update.graph(
                    &mut self.app_state.app_graph.nodes,
                    &self.app_state.hardware_bridge,
                    &self.app_state.app_graph.root_nodes,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{:?}", e);
                        self.app_state.update.clear_cache();
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Text::new("hello").into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(1000)).map(|_| AppMsg::Tick)
    }
}
