use iced::futures::channel;
use iced::{Center, Element, Fill, Left, Right, Theme};
use iced::widget::{button, column, combo_box, pick_list, row, table, text};
use iced_aw::number_input;
use strum::VariantArray;

use crate::device_config::{ChannelConfig, DeviceConfig, InputMode};
use crate::ui::channel_strip;

mod ui;
mod device_config;

fn subtle(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().background.strongest.color),
    }
}

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
                channel_strip(
                    c,
                    channel,
                    subtle,
                    move |config| Message::ChannelConfigChanged(c, config),
                )
        }))
            .width(Fill)
            .height(Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::KanagawaDragon)
        .title(App::title)
        .centered()
        .run()
}
