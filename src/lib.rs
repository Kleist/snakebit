#![no_std]

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

const MAX_HEIGHT: u8 = 5;
const MAX_WIDTH: u8 = 5;

const NORTH_EDGE: u8 = MAX_HEIGHT-1;
const EAST_EDGE: u8 = MAX_WIDTH-1;

pub struct GameState {
    pub snake: Vec<Coord, U32>,
    pub height: u8,
    pub width: u8,
    pub dir: Direction,
}

pub fn next(coord: &Coord, dir: &Direction) -> Option<Coord> {
    use Direction::*;
    match (dir,coord.x,coord.y) {
        (North, _, NORTH_EDGE) => None,
        (East, EAST_EDGE, _) => None,
        (West, 0, _) => None,
        (South, _, 0) => None,
        (North, x, y) => Some(Coord{x,y:y+1}),
        (East, x, y) => Some(Coord{x:x+1,y}),
        (West, x, y) => Some(Coord{x:x-1,y}),
        (South, x, y) => Some(Coord{x,y:y-1}),
    }
}

pub fn step(state: &mut GameState) {
    state.snake[0] = next(&state.snake[0], &state.dir).unwrap()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
