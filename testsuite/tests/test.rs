#![no_std]
#![no_main]

use snakebit as _;

#[defmt_test::tests]
mod tests {
    #[test]
    fn step_north() {
        use heapless::Vec;
        use snakebit::{step, Coord, Direction, GameState};

        let height = 10;
        let width = 10;
        let mut state = GameState {
            height, width,
            snake: Vec::from_slice(&[Coord { x: 2, y: 0 }]).unwrap(),
            dir: Direction::North,
        };

        step(&mut state);

        assert_eq!(Vec::from_slice(&[Coord { x: 2, y: 1 }]), Ok(state.snake));
    }

    #[test]
    fn test_next() {
        use snakebit::{Coord, Direction, next};

        assert_eq!(Coord{x:1,y:2}, next(&Coord{x:1,y:1}, &Direction::North));
        assert_eq!(Coord{x:2,y:1}, next(&Coord{x:1,y:1}, &Direction::East));
        assert_eq!(Coord{x:0,y:1}, next(&Coord{x:1,y:1}, &Direction::West));
        assert_eq!(Coord{x:1,y:0}, next(&Coord{x:1,y:1}, &Direction::South));
    }
}
