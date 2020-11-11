#![no_std]
#![no_main]

#[defmt_test::tests]
mod tests {
    #[test]
    fn test_next() {
        use snakebit::{Coord, Direction, next};

        assert_eq!(Some(Coord{x:1,y:2}), next(&Coord{x:1,y:1}, &Direction::North));
        assert_eq!(Some(Coord{x:2,y:1}), next(&Coord{x:1,y:1}, &Direction::East));
        assert_eq!(Some(Coord{x:0,y:1}), next(&Coord{x:1,y:1}, &Direction::West));
        assert_eq!(Some(Coord{x:1,y:0}), next(&Coord{x:1,y:1}, &Direction::South));
    }

    #[test]
    fn test_next_outside() {
        use snakebit::{Coord, Direction, next};

        assert_eq!(None, next(&Coord{x:2,y:4}, &Direction::North));
        assert_eq!(None, next(&Coord{x:4,y:2}, &Direction::East));
        assert_eq!(None, next(&Coord{x:0,y:2}, &Direction::West));
        assert_eq!(None, next(&Coord{x:2,y:0}, &Direction::South));
    }

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
}
