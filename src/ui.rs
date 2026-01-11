use iced::{Center, Element, Fill, Padding, Pixels};
use iced::widget::{column, pick_list, row, text, text_input, Row};
use iced_aw::number_input;
use std::ops::RangeInclusive;
use strum::VariantArray;

use crate::device_config::{ChannelConfig, InputMode};

pub const SPACING: Pixels = Pixels(8.);
pub const PADDING: Padding = Padding::new(8.);

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
    .spacing(SPACING)
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
            move |value| on_change(channel_clone.with_input_mode(value)),
        )
            .width(Fill),
        match channel.input.mode {
            InputMode::Continuous => column![
                knob(
                    "Min In:",
                    &channel.input.continuous.minimum_input,
                    0..=127,
                    move |value| on_change(channel_clone.with_minimum_input(value)),
                ),
                knob(
                    "Max In:",
                    &channel.input.continuous.maximum_input,
                    0..=127,
                    move |value| on_change(channel_clone.with_maximum_input(value)),
                ),
                knob(
                    "Min Out:",
                    &channel.input.continuous.minimum_output,
                    0..=127,
                    move |value| on_change(channel_clone.with_minimum_output(value)),
                ),
                knob(
                    "Max Out:",
                    &channel.input.continuous.maximum_output,
                    0..=127,
                    move |value| on_change(channel_clone.with_maximum_output(value)),
                ),
                knob(
                    "Drive:",
                    &channel.input.continuous.drive,
                    0..=127,
                    move |value| on_change(channel_clone.with_drive(value)),
                ),
            ],
            _ => column![
                knob(
                    "Released Val:",
                    &channel.input.switch.released_value,
                    0..=127,
                    move |value| on_change(channel_clone.with_released_value(value)),
                ),
                knob(
                    "Pressed Val:",
                    &channel.input.switch.pressed_value,
                    0..=127,
                    move |value| on_change(channel_clone.with_pressed_value(value)),
                ),
            ],
        }
            .spacing(SPACING)
            .align_x(Center)
            .width(Fill)
            .height(Fill),
        knob(
            "CC:",
            &channel.cc,
            1..=128,
            move |value| on_change(channel_clone.with_cc(value)),
        ),
        text_input("Label", channel.label_as_str())
            .on_input(move |label_str| on_change(channel_clone.with_label_str(&label_str)))
            .width(Fill),
    ]
        .spacing(SPACING)
        .align_x(Center)
        .width(200)
        .height(Fill)
        .into()
}
