#![no_std]
#![no_main]

use snakebit as _;

#[defmt_test::tests]
mod tests {
    #[test]
    fn steps_north() {
        use heapless::Vec;
        use snakebit::{step, Coord, Direction, GameState};

        let mut state = GameState {
            snake: Vec::from_slice(&[Coord { x: 2, y: 0 }]).unwrap(),
            dir: Direction::North,
        };

        step(&mut state);

        assert!(state.snake.len() == 1);
        assert_eq!(Vec::from_slice(&[Coord { x: 2, y: 1 }]), Ok(state.snake));
    }
}
