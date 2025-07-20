use embassy_nrf::gpio::{Input, Output};
use embassy_time::Timer;
use defmt::info;
use crate::keycodes::KeyCode;
use crate::layout::Layout;

pub struct Matrix<'a, const N_COLS: usize, const N_ROWS: usize> {
    cols: [Output<'a>; N_COLS],
    rows: [Input<'a>; N_ROWS],
    previous_state: [[bool; N_COLS]; N_ROWS],
}

impl <'a, const N_COLS: usize, const N_ROWS: usize>Matrix<'a, N_COLS, N_ROWS> {
    pub fn new(cols: [Output<'a>; N_COLS], rows: [Input<'a>; N_ROWS]) -> Self {
        Self { 
            cols, 
            rows,
            previous_state: [[false; N_COLS]; N_ROWS],
        }
    }

    pub async fn scan_keys<F>(&mut self, layout: &Layout<N_COLS, N_ROWS>, mut on_key_press: F)
    where
        F: FnMut(KeyCode),
    {
        for (i, col) in self.cols.iter_mut().enumerate() {
            col.set_high();
            // Small delay to allow voltage to stabilize
            Timer::after_micros(10).await;
            
            for (j, row) in self.rows.iter().enumerate() {
                let is_pressed = row.is_high();
                let was_pressed = self.previous_state.get(j).and_then(|row| row.get(i)).copied().unwrap_or(false);
                
                // Only trigger on key press (not release or held)
                if is_pressed && !was_pressed && j < layout.len() && i < layout[j].len() {
                    let keycode = layout[j][i];
                    info!("Key pressed at ({}, {}): {:?}", j, i, keycode);
                    on_key_press(keycode);
                }
                
                // Update the previous state
                if let Some(row_state) = self.previous_state.get_mut(j) {
                    if let Some(cell) = row_state.get_mut(i) {
                        *cell = is_pressed;
                    }
                }
            }
            col.set_low();
        }
        
        // Scan interval - adjust this for responsiveness vs power consumption
        Timer::after_millis(10).await;
    }
}
