use iced::{Center, Element, Fill};
use iced::widget::{column, row};

use crate::device_config::{ChannelConfig, DeviceConfig};
use crate::ui::{PADDING, SPACING, channel_strip};
use crate::theme::expresso_theme;

mod theme;
mod ui;
mod device_config;

#[derive(Debug, Clone)]
enum Message {
    ChannelConfigChanged(usize, ChannelConfig),
}

#[derive(Default, Debug)]
struct App {
    device_config: DeviceConfig<4>,
}

impl App {
    fn title(&self) -> String {
        format!("Midi Expressor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChannelConfigChanged(channel, config) => {
                self.device_config.channels[channel] = config;
            },
        }
    }

    fn view(&self) -> Element<Message> {
        row(self.device_config.channels
            .iter()
            .enumerate()
            .map(|(c, channel)| {
                column![
                    channel_strip(
                        c,
                        channel,
                        move |config| Message::ChannelConfigChanged(c, config),
                    )
                ]
                    .align_x(Center)
                    .width(Fill)
                    .height(Fill)
                    .into()
        }))
            .padding(PADDING)
            .spacing(SPACING)
            .width(Fill)
            .height(Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(expresso_theme())
        .title(App::title)
        .centered()
        .run()
}
