use embassy_usb::{class::hid::{HidWriter, ReportId, RequestHandler}, control::OutResponse, Handler};
use embassy_time;
use usbd_hid::descriptor::KeyboardReport;
use defmt::{info, warn};
use crate::keycodes::KeyCode;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct UsbKeyboard<'d, D: embassy_usb::driver::Driver<'d>, const N: usize> {
    writer: HidWriter<'d, D, N>,
    configured: &'d AtomicBool,
}

impl<'d, D: embassy_usb::driver::Driver<'d>, const N: usize> UsbKeyboard<'d, D, N> {
    pub fn new(writer: HidWriter<'d, D, N>, configured: &'d AtomicBool) -> Self {
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


pub struct UsbRequestHandler {}

impl RequestHandler for UsbRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

pub struct UsbHandler<'d> {
    configured: &'d AtomicBool,
    suspended: &'d AtomicBool,
}

impl <'d>UsbHandler<'d> {
    pub fn new(configured: &'d AtomicBool, suspended: &'d AtomicBool) -> Self {
        Self { configured, suspended }
    }

    pub fn is_suspended(&self) -> bool {
        self.suspended.load(Ordering::Acquire)
    }
}

impl <'d>Handler for UsbHandler<'d> {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        self.suspended.store(false, Ordering::Release);
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

    fn suspended(&mut self, suspended: bool) {
        if suspended {
            info!("Device suspended, the Vbus current limit is 500µA (or 2.5mA for high-power devices with remote wakeup enabled).");
            self.suspended.store(true, Ordering::Release);
        } else {
            self.suspended.store(false, Ordering::Release);
            if self.configured.load(Ordering::Relaxed) {
                info!("Device resumed, it may now draw up to the configured current limit from Vbus");
            } else {
                info!("Device resumed, the Vbus current limit is 100mA");
            }
        }
    }
}