use embassy_usb::class::hid::HidWriter;
use embassy_time;
use usbd_hid::descriptor::KeyboardReport;
use defmt::warn;
use crate::keycodes::KeyCode;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct UsbKeyboard<'d, D: embassy_usb::driver::Driver<'d>, const N: usize> {
    writer: HidWriter<'d, D, N>,
    configured: &'static AtomicBool,
}

impl<'d, D: embassy_usb::driver::Driver<'d>, const N: usize> UsbKeyboard<'d, D, N> {
    pub fn new(writer: HidWriter<'d, D, N>, configured: &'static AtomicBool) -> Self {
        Self { writer, configured }
    }

    pub async fn send_key_report(&mut self, keycode: KeyCode) {
        // Check if USB device is configured before sending reports
        if !self.configured.load(Ordering::Relaxed) {
            warn!("USB device not configured, skipping key report");
            return;
        }

        let (modifier_byte, normal_key) = keycode.to_hid_values();
        
        let report =    KeyboardReport {
            keycodes: [normal_key, 0, 0, 0, 0, 0],
            leds: 0,
            modifier: modifier_byte,
            reserved: 0,
        };
        
        match self.writer.write_serialize(&report).await {
            Ok(()) => {}
            Err(e) => warn!("Failed to send report: {:?}", e),
        };
        
        // Send a key release report after the key press
        // This ensures the key doesn't get "stuck"
        embassy_time::Timer::after_millis(10).await;
        
        let release_report = KeyboardReport {
            keycodes: [0, 0, 0, 0, 0, 0],
            leds: 0,
            modifier: 0,
            reserved: 0,
        };
        
        match self.writer.write_serialize(&release_report).await {
            Ok(()) => {}
            Err(e) => warn!("Failed to send release report: {:?}", e),
        };
    }
}
