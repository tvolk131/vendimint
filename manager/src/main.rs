#![deny(clippy::pedantic, clippy::nursery)]

use iced::widget::{Space, Theme};
use iced::window::settings::PlatformSpecific;
use iced::window::Settings;
use iced::{Element, Length, Size, Task};

fn main() -> iced::Result {
    iced::application("Vendimint Manager", App::update, App::view)
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
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct App {}

impl App {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {}
    }

    pub fn view(&self) -> Element<Message> {
        Space::new(Length::Fill, Length::Fill).into()
    }
}
