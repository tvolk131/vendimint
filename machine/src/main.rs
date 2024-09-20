#![deny(clippy::pedantic, clippy::nursery)]

use iced::widget::{Space, Theme};
use iced::window::settings::PlatformSpecific;
use iced::window::{self, Settings};
use iced::{Element, Length, Size, Task};

fn main() -> iced::Result {
    iced::application("Vendimint Machine", App::update, App::view)
        .theme(|_| Theme::Dark)
        .window(Settings {
            size: Size {
                width: 800.0,
                height: 600.0,
            },
            position: iced::window::Position::Default,
            min_size: Some(Size {
                width: 600.0,
                height: 400.0,
            }),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: iced::window::Level::Normal,
            icon: None,                                     // TODO: Set icon.
            platform_specific: PlatformSpecific::default(), // TODO: Set platform specific settings for each platform.
            exit_on_close_request: true,
        })
        .run_with(App::init)
}

#[derive(Debug, Clone)]
enum Message {
    SetWindowMode(window::Mode),
}

#[derive(Default)]
struct App {}

impl App {
    fn init() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::done(Message::SetWindowMode(window::Mode::Fullscreen)),
        )
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::SetWindowMode(mode) => {
                window::get_latest().and_then(move |window| window::change_mode(window, mode))
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        Space::new(Length::Fill, Length::Fill).into()
    }
}
