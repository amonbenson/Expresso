use iced::{Center, Element, Fill};
use iced::widget::{column, pick_list, row, text, Row};
use iced_aw::number_input;
use std::ops::RangeInclusive;
use strum::VariantArray;

use crate::device_config::{ChannelConfig, InputMode};

pub fn knob<'a, Message: Clone + 'a, F>(
    label: &'a str,
    value: &'a u8,
    range: RangeInclusive<u8>,
    on_change: F,
) -> Row<'a, Message>
where
    F: Fn(u8) -> Message + Copy + 'static,
{
    row![
        text(label)
            .width(Fill),
        number_input(value, range, on_change)
            .width(Fill),
    ]
    .spacing(4)
    .align_y(Center)
    .width(Fill)
}

pub fn channel_strip<'a, Message: Clone + 'a, F>(
    channel_index: usize,
    channel: &'a ChannelConfig,
    text_style: F,
    on_change: impl Fn(ChannelConfig) -> Message + Copy + 'static,
) -> Element<'a, Message>
where
    F: Fn(&iced::Theme) -> text::Style + Copy + 'static,
{
    let channel_clone = channel.clone();

    column![
        text((channel_index + 1).to_string())
            .size(36)
            .style(text_style),
        pick_list(
            InputMode::VARIANTS,
            Some(&channel.input.mode),
            {
                let channel_clone = channel_clone.clone();
                move |mode| {
                    let mut new_config = channel_clone.clone();
                    new_config.input.mode = mode;
                    on_change(new_config)
                }
            },
        ),
        match channel.input.mode {
            InputMode::Continuous => column![
                knob(
                    "Min In:",
                    &channel.input.continuous.minimum_input,
                    0..=127,
                    {
                        let channel_clone = channel_clone.clone();
                        move |value| {
                            let mut new_config = channel_clone.clone();
                            new_config.input.continuous.minimum_input = value;
                            on_change(new_config)
                        }
                    }
                ),
                knob(
                    "Max In:",
                    &channel.input.continuous.maximum_input,
                    0..=127,
                    {
                        let channel_clone = channel_clone.clone();
                        move |value| {
                            let mut new_config = channel_clone.clone();
                            new_config.input.continuous.maximum_input = value;
                            on_change(new_config)
                        }
                    }
                ),
                knob(
                    "Min Out:",
                    &channel.input.continuous.minimum_output,
                    0..=127,
                    {
                        let channel_clone = channel_clone.clone();
                        move |value| {
                            let mut new_config = channel_clone.clone();
                            new_config.input.continuous.minimum_output = value;
                            on_change(new_config)
                        }
                    }
                ),
                knob(
                    "Max Out:",
                    &channel.input.continuous.maximum_output,
                    0..=127,
                    {
                        let channel_clone = channel_clone.clone();
                        move |value| {
                            let mut new_config = channel_clone.clone();
                            new_config.input.continuous.maximum_output = value;
                            on_change(new_config)
                        }
                    }
                ),
                knob(
                    "Drive:",
                    &channel.input.continuous.drive,
                    0..=127,
                    {
                        let channel_clone = channel_clone.clone();
                        move |value| {
                            let mut new_config = channel_clone.clone();
                            new_config.input.continuous.drive = value;
                            on_change(new_config)
                        }
                    }
                ),
            ]
                .align_x(Center)
                .width(Fill)
                .height(Fill),
            _ => column![
                knob(
                    "Released Val:",
                    &channel.input.switch.released_value,
                    0..=127,
                    {
                        let channel_clone = channel_clone.clone();
                        move |value| {
                            let mut new_config = channel_clone.clone();
                            new_config.input.switch.released_value = value;
                            on_change(new_config)
                        }
                    }
                ),
                knob(
                    "Pressed Val:",
                    &channel.input.switch.pressed_value,
                    0..=127,
                    {
                        let channel_clone = channel_clone.clone();
                        move |value| {
                            let mut new_config = channel_clone.clone();
                            new_config.input.switch.pressed_value = value;
                            on_change(new_config)
                        }
                    }
                ),
            ]
                .align_x(Center)
                .width(Fill)
                .height(Fill),
        },
        row![
            text("CC:")
                .width(Fill),
            number_input(
                &(channel.cc + 1),
                1..=128,
                {
                    let channel_clone = channel_clone.clone();
                    move |value| {
                        let mut new_config = channel_clone.clone();
                        new_config.cc = value - 1;
                        on_change(new_config)
                    }
                }
            )
                .width(Fill),
        ]
            .spacing(4)
            .align_y(Center)
            .width(Fill),
    ]
        .align_x(Center)
        .width(Fill)
        .height(Fill)
        .into()
}
