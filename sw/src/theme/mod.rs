pub mod config;
pub mod widget;

use iced::Theme;

pub fn theme() -> Theme {
    Theme::custom(String::from("Expresso"), config::PALETTE)
}
