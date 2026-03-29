#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::{
    InputPin,
    OutputPin
};
use microbit::board::Board;
 
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut col = board.display_pins.col1;
    let mut row = board.display_pins.row1;
    let mut button_b = board.buttons.button_b;
    let mut button_a = board.buttons.button_a;

    col.set_low().unwrap();

    loop {
        if button_b.is_low().unwrap() || button_a.is_low().unwrap() {
            row.set_high().unwrap();
        } else {
            row.set_low().unwrap();
        }
    }
}