use defmt::Format;
use usbd_hid::descriptor::KeyboardUsage;

#[repr(u8)]
#[allow(unused)]
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Eq, PartialEq, Format)]
pub enum Extra {
    NA,
}

#[repr(u8)]
#[allow(unused)]
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Eq, PartialEq, Format)]
pub enum MacosKeys {
    Fn = 0xA4,    // Custom scancode for Fn (no standard exists)
}

#[repr(u8)]
#[allow(unused)]
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Eq, PartialEq, Format)]
pub enum KeyCode {
    Base(KeyboardUsage),
    Macos(MacosKeys),
    Extra(Extra),
}

impl KeyCode {
    /// Converts a KeyCode to its corresponding HID usage code
    pub fn to_usage_code(&self) -> u8 {
        match self {
            KeyCode::Base(usage) => *usage as u8,
            KeyCode::Macos(macos_key) => *macos_key as u8,
            KeyCode::Extra(extra) => *extra as u8,
        }
    }
    
    /// Determines if the keycode is a modifier key and returns the appropriate
    /// modifier byte and normal key values for the HID report
    pub fn to_hid_values(&self) -> (u8, u8) {
        let keycode = self.to_usage_code();
        
        if keycode >= 0xE0 && keycode <= 0xE7 {
            // It's a modifier key
            let modifier_bit = 1 << (keycode - 0xE0);
            (modifier_bit, 0)
        } else {
            // It's a normal key
            (0, keycode)
        }
    }
}
