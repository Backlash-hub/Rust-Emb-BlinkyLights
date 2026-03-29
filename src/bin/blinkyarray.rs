#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{
    InputPin,
    OutputPin
};
use microbit::{
    board::Board,
    hal::Timer,
};

use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut button_b = board.buttons.button_b;
    let mut button_a = board.buttons.button_a;

    let col1 = board.display_pins.col1.degrade();
    let col2 = board.display_pins.col2.degrade();
    let col3 = board.display_pins.col3.degrade();
    let col4 = board.display_pins.col4.degrade();
    let col5 = board.display_pins.col5.degrade();
    let row1 = board.display_pins.row1.degrade();
    let row2 = board.display_pins.row2.degrade();
    let row3 = board.display_pins.row3.degrade();
    let row4 = board.display_pins.row4.degrade();
    let row5 = board.display_pins.row5.degrade();

    let mut col = [col1, col2, col3, col4, col5];
    let mut row = [row1, row2, row3, row4, row5];

    for col in col.iter_mut() {
        col.set_high().unwrap();
    }

    for row in row.iter_mut() {
        row.set_low().unwrap();
    }

    loop {
        if button_b.is_low().unwrap() || button_a.is_low().unwrap() {
            for row in row.iter_mut() {
                for col in col.iter_mut() {
                    col.set_low().unwrap();
                    row.set_high().unwrap();
                    timer.delay_ms(100);
                    row.set_low().unwrap();
                    col.set_high().unwrap();
                }
            }
        }
    }
}