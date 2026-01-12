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
use embassy_usb::{Builder, Handler};
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
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
    config.manufacturer = Some("Amon Benson");
    config.product = Some("Midi Expressor");
    config.serial_number = Some("1337");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // Microsoft OS descriptor??
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut request_handler = MyRequestHandler {};
    let mut device_handler = MyDeviceHandler::new();

    let mut channel_strips = [ChannelStrip::default(); NUM_CHANNELS];

    let mut state = State::new();
    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    builder.handler(&mut device_handler);

    // Create classes on the builder.
    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 8,
    };

    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, config);

    let mut usb = builder.build();
    let usb_fut = usb.run();
    let (reader, mut writer) = hid.split();

    let in_fut = async {
        loop {
            for (i, channel_strip) in channel_strips.iter_mut().enumerate() {
                let raw_value = adc.blocking_read(&mut adc_channels[i], SampleTime::CYCLES24_5);

                channel_strip.process(raw_value);

                if channel_strip.changed() {
                    info!("Channel {}: Value = {}", i, channel_strip.value())
                }
            }
        }
    };

    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    join(usb_fut, join(in_fut, out_fut)).await;
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: ReportId, buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, duration_ms: u32) {
        info!("Set idle rate for {:?} to {:?}", id, duration_ms)
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);

        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!("Device configured, it may now draw up to the configured current limit from Vbus.")
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
