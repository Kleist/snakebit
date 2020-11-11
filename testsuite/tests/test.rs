#![no_std]
#![no_main]

use snakebit as _;

#[defmt_test::tests]
mod game_logic {
    #[test]
    fn steps_in_direction() {
        use heapless::Vec;
        use snakebit::{step, Coord, Direction, GameState};

        let mut state = GameState {
            snake: Vec::new(),
            dir: Direction::North,
        };

        let _ = state.snake.push(Coord { x: 2, y: 0 });
        step(&mut state);
        assert!(state.snake.len() == 1);
        assert_eq!(Coord { x: 2, y: 1 }, state.snake[0]);
    }
}
