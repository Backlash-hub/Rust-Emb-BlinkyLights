#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use microbit::{
    board::Board,
    hal::Timer,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut col = board.display_pins.col1;
    let mut row = board.display_pins.row1;

    col.set_low().unwrap();

    loop {
        row.set_high().unwrap();
        timer.delay_ms(500);
        row.set_low().unwrap();
        timer.delay_ms(10000);
    }
}