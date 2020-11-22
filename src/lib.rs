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

const SPEED_STEPS: u8 = 10; // Approx Smaller is faster - speed ~= 16Hz/SPEED_STEPS steps/second
const RESTART_STEPS: u8 = 50; // Restart after so many steps ~= SPEED_STEPS/16Hz seconds

pub struct GameState {
    pub snake: Vec<Coord, U32>,
    pub dir: Direction,
    pub tick: u8,
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

impl GameState {
    pub fn new() -> GameState {
        GameState {
            snake: Vec::from_slice(&[Coord{x: 2, y: 0}]).unwrap(),
            dir: Direction::North,
            tick: 0,
        }
    }

    fn restart(&mut self){
        defmt::info!("Restarting game");
        *self = GameState::new();
    }

    pub fn game_tick(&mut self) {
        self.tick += 1;
        if self.tick == SPEED_STEPS {
            if self.step() {
                self.tick = 0;
            } else {
                defmt::info!("Game over");
            }
        }
        else if self.tick == RESTART_STEPS {
            self.restart();
        }
        else if self.tick % 5 == 0 {
            defmt::info!("tick: {:?}", self.tick);
        }
    }

    pub fn step(&mut self) -> bool {
        if let Some(new_coord) = next(&self.snake[0], &self.dir) {
            let len = self.snake.len();
            for i in 1..len {
                if new_coord == self.snake[len-i-1] {
                    defmt::info!("Ran over tail");
                    return false;
                }
                self.snake[len-i] = self.snake[len-i-1];
            }
            self.snake[0] = new_coord;
            return true;
        }
        
        defmt::info!("Ran out of bounds");
        return false;
    }

    pub fn turn_left(&mut self) {
        use Direction::*;
        self.dir = match self.dir {
            North => West,
            West => South,
            South => East,
            East => North
        }
    }
    
    pub fn turn_right(&mut self) {
        use Direction::*;
        self.dir = match self.dir {
            West => North,
            South => West,
            East => South,
            North => East
        }
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
