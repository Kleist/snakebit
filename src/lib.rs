#![no_std]

use defmt_rtt as _; // global logger
use panic_probe as _;

use microbit::hal::nrf51 as _;
use microbit::display::image::BitImage;

use heapless::consts::U32;
use heapless::Vec;

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

pub fn step(state: &mut GameState) -> bool {
    if let Some(new_coord) = next(&state.snake[0], &state.dir) {
        let len = state.snake.len();
        for i in 1..len {
            if new_coord == state.snake[len-i-1] {
                defmt::info!("Ran over tail");
                return false;
            }
            state.snake[len-i] = state.snake[len-i-1];
        }
        state.snake[0] = new_coord;
        return true;
    }
    
    defmt::info!("Ran out of bounds");
    return false;
}

pub fn turn_left(state: &mut GameState) {
    use Direction::*;
    state.dir = match state.dir {
        North => West,
        West => South,
        South => East,
        East => North
    }
}

pub fn turn_right(state: &mut GameState) {
    use Direction::*;
    state.dir = match state.dir {
        West => North,
        South => West,
        East => South,
        North => East
    }
}

pub fn render(snake: &[Coord]) -> BitImage {
    let mut frame = [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0]
    ];
    for coord in snake.iter() {
        frame[coord.y as usize][coord.x as usize] = 1;
    }
    BitImage::new(&frame)
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
