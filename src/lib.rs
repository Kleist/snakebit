#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

use defmt_rtt as _; // global logger
use panic_probe as _;

use microbit::hal::nrf51 as _;

use heapless::Vec;
use heapless::consts::U32;

pub enum Direction {
    North,
    West,
    South,
    East
}

#[derive(Clone, Debug, PartialEq)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

pub struct GameState {
    pub snake: Vec<Coord, U32>,
    pub dir: Direction
}

pub fn step(state: &mut GameState) {
    state.snake[0].y = 1
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
