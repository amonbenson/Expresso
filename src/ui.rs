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
    on_input_mode_change: impl Fn(InputMode) -> Message + Copy + 'static,
    on_released_value_change: impl Fn(u8) -> Message + Copy + 'static,
    on_pressed_value_change: impl Fn(u8) -> Message + Copy + 'static,
    on_min_input_change: impl Fn(u8) -> Message + Copy + 'static,
    on_max_input_change: impl Fn(u8) -> Message + Copy + 'static,
    on_min_output_change: impl Fn(u8) -> Message + Copy + 'static,
    on_max_output_change: impl Fn(u8) -> Message + Copy + 'static,
    on_drive_change: impl Fn(u8) -> Message + Copy + 'static,
    on_cc_change: impl Fn(u8) -> Message + Copy + 'static,
) -> Element<'a, Message>
where
    F: Fn(&iced::Theme) -> text::Style + Copy + 'static,
{
    column![
        text((channel_index + 1).to_string())
            .size(36)
            .style(text_style),
        pick_list(
            InputMode::VARIANTS,
            Some(&channel.input.mode),
            on_input_mode_change,
        ),
        match channel.input.mode {
            InputMode::Continuous => column![
                knob(
                    "Min In:",
                    &channel.input.continuous.minimum_input,
                    0..=127,
                    on_min_input_change
                ),
                knob(
                    "Max In:",
                    &channel.input.continuous.maximum_input,
                    0..=127,
                    on_max_input_change
                ),
                knob(
                    "Min Out:",
                    &channel.input.continuous.minimum_output,
                    0..=127,
                    on_min_output_change
                ),
                knob(
                    "Max Out:",
                    &channel.input.continuous.maximum_output,
                    0..=127,
                    on_max_output_change
                ),
                knob(
                    "Drive:",
                    &channel.input.continuous.drive,
                    0..=127,
                    on_drive_change
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
                    on_released_value_change
                ),
                knob(
                    "Pressed Val:",
                    &channel.input.switch.pressed_value,
                    0..=127,
                    on_pressed_value_change
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
                on_cc_change
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
