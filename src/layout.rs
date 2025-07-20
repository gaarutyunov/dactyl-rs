use usbd_hid::descriptor::KeyboardUsage;

use crate::keycodes::{Extra, KeyCode};

pub type Layout<const N_COLS: usize, const N_ROWS: usize> = [[KeyCode; N_COLS]; N_ROWS];

// Macros for shorter keycode definitions
macro_rules! k {
    ($key:ident) => {
        KeyCode::Base(KeyboardUsage::$key)
    };
}

macro_rules! na {
    () => {
        KeyCode::Extra(Extra::NA)
    };
}

pub fn get_left_layout() -> Layout<7, 6> {
    [
        [na!(), na!(), na!(), na!(), na!(), na!(), na!()],
        [na!(), k!(KeyboardQq), k!(KeyboardWw), k!(KeyboardEe), k!(KeyboardRr), k!(KeyboardTt), na!()],
        [na!(), k!(KeyboardAa), k!(KeyboardSs), k!(KeyboardDd), k!(KeyboardFf), k!(KeyboardGg), na!()],
        [na!(), k!(KeyboardZz), k!(KeyboardXx), k!(KeyboardCc), k!(KeyboardVv), k!(KeyboardBb), na!()],
        [na!(), na!(), na!(), na!(), na!(), na!(), na!()],
        [na!(), na!(), na!(), na!(), na!(), na!(), na!()],
    ]
}

pub fn get_right_layout() -> Layout<7, 6> {
    [
        [na!(), na!(), na!(), na!(), na!(), na!(), na!()],
        [na!(), k!(KeyboardYy), k!(KeyboardUu), k!(KeyboardIi), k!(KeyboardOo), k!(KeyboardPp), na!()],
        [na!(), k!(KeyboardHh), k!(KeyboardJj), k!(KeyboardKk), k!(KeyboardLl), na!(), na!()],
        [na!(), k!(KeyboardNn), k!(KeyboardMm), k!(KeyboardCommaLess), k!(KeyboardPeriodGreater), k!(KeyboardSlashQuestion), na!()],
        [na!(), na!(), na!(), na!(), na!(), na!(), na!()],
        [na!(), na!(), na!(), na!(), na!(), na!(), na!()],
    ]
}
