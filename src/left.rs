#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use dactyl_rs::{
    keycodes::KeyCode,
    layout::get_left_layout as get_default_layout,
    matrix::Matrix,
    usb::{UsbHandler, UsbKeyboard, UsbRequestHandler},
};
use defmt::{info, unwrap};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::{
    join::join4,
    select::{Either, select},
};
use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Level, Output, OutputDrive, Pull},
    pac, peripherals, usb as nrf_usb,
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, signal::Signal,
};
use panic_probe as _;
use usbd_hid::descriptor::SerializedDescriptor;

bind_interrupts!(struct Irqs {
    USBD => nrf_usb::InterruptHandler<peripherals::USBD>;
    CLOCK_POWER => nrf_usb::vbus_detect::InterruptHandler;
});

static SUSPENDED: AtomicBool = AtomicBool::new(false);
static USB_CONFIGURED: AtomicBool = AtomicBool::new(false);
static KEY_CHANNEL: Channel<CriticalSectionRawMutex, KeyCode, 16> = Channel::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Add early logging to test defmt
    defmt::info!("=== Dactyl keyboard firmware starting ===");
    // Enable the external high-frequency oscillator (hfosc)
    // This is necessary for USB to work correctly.
    // The hfosc is used as the clock source for the USB peripheral.
    info!("Enabling External HFOSC...");
    pac::CLOCK.tasks_hfclkstart().write_value(1);
    while pac::CLOCK.events_hfclkstarted().read() != 1 {}
    info!("External HFOSC enabled successfully");

    // Initialize USB - try software VBUS detection to bypass hardware issues
    let driver = embassy_nrf::usb::Driver::new(
        p.USBD,
        Irqs,
        embassy_nrf::usb::vbus_detect::HardwareVbusDetect::new(Irqs),
    );

    // Add a small delay and check USB status
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("German Arutyunov");
    config.product = Some("Dactyal Manuform");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;
    config.supports_remote_wakeup = true;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut request_handler = UsbRequestHandler {};
    let mut device_handler = UsbHandler::new(&USB_CONFIGURED, &SUSPENDED);

    let mut state = embassy_usb::class::hid::State::new();

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    builder.handler(&mut device_handler);

    // Create HID class
    let hid_config = embassy_usb::class::hid::Config {
        report_descriptor: usbd_hid::descriptor::KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let hid = embassy_usb::class::hid::HidReaderWriter::<_, 1, 8>::new(
        &mut builder,
        &mut state,
        hid_config,
    );
    let mut usb_device = builder.build();
    let (reader, writer) = hid.split();

    // Initialize keyboard
    let mut keyboard = UsbKeyboard::new(writer, &USB_CONFIGURED);

    // Create a channel for sending key events from matrix scanner to USB task
    let key_sender = KEY_CHANNEL.sender();
    let key_receiver = KEY_CHANNEL.receiver();

    // Initialize matrix scanner
    let cols = [
        Output::new(p.P0_31, Level::Low, OutputDrive::Standard), // col 0
        Output::new(p.P0_29, Level::Low, OutputDrive::Standard), // col 1
        Output::new(p.P0_02, Level::Low, OutputDrive::Standard), // col 2
        Output::new(p.P1_13, Level::Low, OutputDrive::Standard), // col 3
        Output::new(p.P0_03, Level::Low, OutputDrive::Standard), // col 4
        Output::new(p.P0_28, Level::Low, OutputDrive::Standard), // col 5
        Output::new(p.P1_11, Level::Low, OutputDrive::Standard), // col 6
    ];

    let rows = [
        Input::new(p.P0_20, Pull::Down), // row 0
        Input::new(p.P0_13, Pull::Down), // row 1
        Input::new(p.P0_24, Pull::Down), // row 2
        Input::new(p.P0_09, Pull::Down), // row 3
        Input::new(p.P0_10, Pull::Down), // row 4
        Input::new(p.P1_06, Pull::Down), // row 4
    ];

    let mut matrix = Matrix::new(cols, rows);
    let layout = get_default_layout();

    let remote_wakeup: Signal<CriticalSectionRawMutex, ()> = Signal::new();

    let usb_fut = async {
        info!("USB task starting...");
        loop {
            info!("USB device starting enumeration...");
            usb_device.run_until_suspend().await;
            info!("USB device suspended");
            match select(usb_device.wait_resume(), remote_wakeup.wait()).await {
                Either::First(_) => {
                    info!("USB device resumed");
                }
                Either::Second(_) => {
                    info!("Remote wakeup triggered");
                    unwrap!(usb_device.remote_wakeup().await)
                }
            }
        }
    };

    let in_fut = async {
        loop {
            matrix
                .scan_keys(&layout, |keycode| {
                    if SUSPENDED.load(Ordering::Relaxed) {
                        info!("Triggering remote wakeup");
                        remote_wakeup.signal(());
                    } else {
                        // Send keycode through channel to USB task
                        info!("Key pressed: {:?}", keycode.to_usage_code());
                        if key_sender.try_send(keycode).is_err() {
                            info!("Key channel full, dropping keycode");
                        }
                    }
                })
                .await;
        }
    };

    let keyboard_fut = async {
        loop {
            // Wait for key events from the channel
            let keycode = key_receiver.receive().await;
            keyboard.send_key_report(keycode).await;
        }
    };

    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    join4(usb_fut, in_fut, keyboard_fut, out_fut).await;
}
