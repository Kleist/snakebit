#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

use defmt_rtt as _; // global logger
use panic_probe as _;

use microbit::hal::nrf51 as _;

use heapless::consts::U32;
use heapless::Vec;

pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

pub struct GameState {
    pub snake: Vec<Coord, U32>,
    pub height: u8,
    pub width: u8,
    pub dir: Direction,
}

pub fn next(coord: &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::North => Coord {
            x: coord.x,
            y: coord.y + 1,
        },
        Direction::West => Coord {
            x: coord.x - 1,
            y: coord.y,
        },
        Direction::East => Coord {
            x: coord.x + 1,
            y: coord.y,
        },
        Direction::South => Coord {
            x: coord.x,
            y: coord.y - 1,
        },
    }
}

pub fn step(state: &mut GameState) {
    state.snake[0] = next(&state.snake[0], &state.dir)
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
