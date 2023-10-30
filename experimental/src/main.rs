#![allow(dead_code)]

use iced::widget::{Button, Column, Container, Row, Space, TextInput};
use iced::{self, executor, widget::Text, Application, Command, Settings};
use iced::{time, Length};

use std::time::Duration;

struct AppState {
    // goal:
    // - mutate and access in app
    // - access in subscription
    name: String,

    // goal:
    // - mutate and access in app
    // - mutate and access in subscription
    number: i32,
}

pub struct Ui {
    app_state: AppState,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Tick,
    ChangeName(String),
    ChangeNumber(i32),
}

impl Application for Ui {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app_state = AppState {
            name: "name1".into(),
            number: 0,
        };

        let ui_state = Ui { app_state };

        (ui_state, Command::none())
    }

    fn title(&self) -> String {
        String::from("App")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            AppMsg::Tick => {
                println!("tick")
            }
            AppMsg::ChangeName(name) => self.app_state.name = name,
            AppMsg::ChangeNumber(number) => self.app_state.number = number,
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let counter = Row::new()
            .push(
                Button::new(Text::new("increment"))
                    .on_press(AppMsg::ChangeNumber(self.app_state.number + 1)),
            )
            .push(Space::new(16, 0))
            .push(Text::new(self.app_state.number.to_string()));

        let content = Column::new()
            .push(
                TextInput::new("", &self.app_state.name).on_input(|text| AppMsg::ChangeName(text)),
            )
            .push(Space::new(0, 30))
            .push(counter)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center);

        Container::new(content).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(1000)).map(|_| AppMsg::Tick)
    }
}

fn main() {
    Ui::run(Settings::default()).unwrap();
}
