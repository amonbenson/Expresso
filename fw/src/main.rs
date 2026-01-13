#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::usb::Driver;
use embassy_stm32::{Config, bind_interrupts, peripherals, usb};
use embassy_stm32::adc::{Adc, AdcChannel, AdcConfig, AnyAdcChannel, Resolution, SampleTime};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_usb::class::midi::MidiClass;
use embassy_usb::driver::EndpointError;
use embassy_usb::{Builder, Handler};
use embassy_usb::control::OutResponse;
use midi_types::MidiMessage;
use midi_types::status::CONTROL_CHANGE;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use crate::channel_strip::ChannelStrip;

use {defmt_rtt as _, panic_probe as _};

mod channel_strip;

const NUM_CHANNELS: usize = 4;

bind_interrupts!(struct Irqs {
    USB_LP => usb::InterruptHandler<peripherals::USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    info!("Initializing LED outputs...");
    let mut led1 = Output::new(p.PB12, Level::Low, Speed::Low);
    let mut led2 = Output::new(p.PB13, Level::Low, Speed::Low);

    info!("Initializing ADC...");
    let config = AdcConfig {
        resolution: Some(Resolution::BITS12),
        ..AdcConfig::default()
    };
    let mut adc = Adc::new(p.ADC1, config);
    let mut adc_channels = [
        p.PA0.degrade_adc(),
        p.PA1.degrade_adc(),
        p.PA2.degrade_adc(),
        p.PA3.degrade_adc(),
    ];

    info!("Initializing USB");
    let driver = Driver::new(
        p.USB,
        Irqs,
        p.PA12,
        p.PA11,
    );

    let mut config = embassy_usb::Config::new(0x1209, 0xd2b3);
    config.manufacturer = Some("schlegelflegel");
    config.product = Some("Midi Expressor");
    config.serial_number = Some("0.3.0");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );
    let mut midi_class = MidiClass::new(&mut builder, 1, 1, 64);
    let mut usb = builder.build();

    let mut channel_strips = [ChannelStrip::default(); NUM_CHANNELS];

    let mut usb_fut = usb.run();

    let mut midi_fut = async {
        loop {
            midi_class.wait_connection().await;
            info!("USB Connected");
            let _ = midi_session(&mut midi_class).await;
            info!("USB Disconnected");
        }
    };

    // let in_fut = async {
    //     loop {
    //         for (i, channel_strip) in channel_strips.iter_mut().enumerate() {
    //             let raw_value = adc.blocking_read(&mut adc_channels[i], SampleTime::CYCLES24_5);

    //             channel_strip.process(raw_value);

    //             if channel_strip.changed() {
    //                 info!("Channel {}: Value = {}", i, channel_strip.value())
    //             }
    //         }
    //     }
    // };

    join(usb_fut, midi_fut).await;
}

pub struct Disconnected;

impl From<EndpointError> for Disconnected {
    fn from(value: EndpointError) -> Self {
        match value {
            EndpointError::BufferOverflow => defmt::panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected,
        }
    }
}

static MIDI_QUEUE: Channel<ThreadModeRawMutex, MidiMessage, 10> = Channel::new();

pub async fn midi_session<'d, T: usb::Instance + 'd>(midi: &mut MidiClass<'d, Driver<'d, T>>) -> Result<(), Disconnected> {
    loop {
        let msg = MIDI_QUEUE.receive().await;
        match msg {
            MidiMessage::ControlChange(channel, control, value) => {
                let cin = CONTROL_CHANGE >> 4;
                let status = CONTROL_CHANGE << 4 | u8::from(channel);
                let packet = [CONTROL_CHANGE >> 4, status, u8::from(control), u8::from(value)];
                midi.write_packet(&packet).await?;
            }
            _ => {}
        }
    }
}
