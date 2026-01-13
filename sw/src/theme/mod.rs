pub mod palette;
pub mod widget;

use iced::Theme;

pub fn expresso_theme() -> Theme {
    Theme::custom(String::from("Expresso Theme"), palette::EXPRESSO_PALETTE)
}
