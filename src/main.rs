use iced::{Center, Element, Fill, Left, Right, Theme};
use iced::widget::{button, column, combo_box, pick_list, row, table, text};
use iced_aw::number_input;
use strum::VariantArray;

use crate::device_config::{DeviceConfig, InputMode};
use crate::ui::channel_strip;

mod ui;
mod device_config;

fn subtle(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().background.strongest.color),
    }
}

#[derive(Debug, Clone)]
enum ChannelMessage {
    InputModeSelected(InputMode),

    ReleasedValueChanged(u8),
    PressedValueChanged(u8),

    MinimumInputChanged(u8),
    MaximumInputChanged(u8),
    MinimumOutputChanged(u8),
    MaximumOutputChanged(u8),
    DriveChanged(u8),

    CcChanged(u8),
    LabelChanged(String),
}

#[derive(Debug, Clone)]
enum Message {
    ChannelMessage(usize, ChannelMessage),
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
            Message::ChannelMessage(c, channel_message) => {
                let channel = &mut self.device_config.channels[c];

                match channel_message {
                    ChannelMessage::InputModeSelected(mode) => {
                        channel.input.mode = mode;
                    },
                    ChannelMessage::ReleasedValueChanged(value) => {
                        channel.input.switch.released_value = value;
                    },
                    ChannelMessage::PressedValueChanged(value) => {
                        channel.input.switch.pressed_value = value;
                    },
                    ChannelMessage::MinimumInputChanged(value) => {
                        channel.input.continuous.minimum_input = value;
                    },
                    ChannelMessage::MaximumInputChanged(value) => {
                        channel.input.continuous.maximum_input = value;
                    },
                    ChannelMessage::MinimumOutputChanged(value) => {
                        channel.input.continuous.minimum_output = value;
                    },
                    ChannelMessage::MaximumOutputChanged(value) => {
                        channel.input.continuous.maximum_output = value;
                    },
                    ChannelMessage::DriveChanged(value) => {
                        channel.input.continuous.drive = value;
                    },
                    ChannelMessage::CcChanged(value) => {
                        channel.cc = value;
                    },
                    ChannelMessage::LabelChanged(value) => {
                        // channel.label.write(value.as_bytes()).unwrap();
                    },
                }
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
                    move |mode| Message::ChannelMessage(c, ChannelMessage::InputModeSelected(mode)),
                    move |value| Message::ChannelMessage(c, ChannelMessage::ReleasedValueChanged(value)),
                    move |value| Message::ChannelMessage(c, ChannelMessage::PressedValueChanged(value)),
                    move |value| Message::ChannelMessage(c, ChannelMessage::MinimumInputChanged(value)),
                    move |value| Message::ChannelMessage(c, ChannelMessage::MaximumInputChanged(value)),
                    move |value| Message::ChannelMessage(c, ChannelMessage::MinimumOutputChanged(value)),
                    move |value| Message::ChannelMessage(c, ChannelMessage::MaximumOutputChanged(value)),
                    move |value| Message::ChannelMessage(c, ChannelMessage::DriveChanged(value)),
                    move |value| Message::ChannelMessage(c, ChannelMessage::CcChanged(value - 1)),
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
