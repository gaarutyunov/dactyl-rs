use crate::keycodes::{Extra, KeyCode, MacosKeys};
use usbd_hid::descriptor::KeyboardUsage;

pub type Layout<const N_COLS: usize, const N_ROWS: usize> = [[KeyCode; N_COLS]; N_ROWS];

pub fn get_default_layout() -> Layout<7, 6> {
    [
        [KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA)],
        [KeyCode::Extra(Extra::NA), KeyCode::Base(KeyboardUsage::KeyboardQq), KeyCode::Base(KeyboardUsage::KeyboardWw), KeyCode::Base(KeyboardUsage::KeyboardEe), KeyCode::Base(KeyboardUsage::KeyboardRr), KeyCode::Base(KeyboardUsage::KeyboardTt), KeyCode::Extra(Extra::NA)],
        [KeyCode::Extra(Extra::NA), KeyCode::Base(KeyboardUsage::KeyboardAa), KeyCode::Base(KeyboardUsage::KeyboardSs), KeyCode::Base(KeyboardUsage::KeyboardDd), KeyCode::Base(KeyboardUsage::KeyboardFf), KeyCode::Base(KeyboardUsage::KeyboardGg), KeyCode::Extra(Extra::NA)],
        [KeyCode::Extra(Extra::NA), KeyCode::Base(KeyboardUsage::KeyboardZz), KeyCode::Base(KeyboardUsage::KeyboardXx), KeyCode::Base(KeyboardUsage::KeyboardCc), KeyCode::Base(KeyboardUsage::KeyboardVv), KeyCode::Base(KeyboardUsage::KeyboardBb), KeyCode::Extra(Extra::NA)],
        [KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA)],
        [KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA), KeyCode::Extra(Extra::NA)],
    ]
}
