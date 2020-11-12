#![no_std]
#![no_main]

pub mod a {
    use snakebit::{Coord, Direction, GameState};
    pub fn state_with_dir(dir: Direction) -> GameState {
        use heapless::Vec;
        GameState {
            snake: Vec::from_slice(&[Coord { x: 1, y: 0 }]).unwrap(),
            dir: dir,
        }
    }
}
#[defmt_test::tests]
mod tests {
    #[test]
    fn test_next() {
        use snakebit::{next, Coord, Direction};

        assert_eq!(
            Some(Coord { x: 1, y: 2 }),
            next(&Coord { x: 1, y: 1 }, &Direction::North)
        );
        assert_eq!(
            Some(Coord { x: 2, y: 1 }),
            next(&Coord { x: 1, y: 1 }, &Direction::East)
        );
        assert_eq!(
            Some(Coord { x: 0, y: 1 }),
            next(&Coord { x: 1, y: 1 }, &Direction::West)
        );
        assert_eq!(
            Some(Coord { x: 1, y: 0 }),
            next(&Coord { x: 1, y: 1 }, &Direction::South)
        );
    }

    #[test]
    fn test_next_outside() {
        use snakebit::{next, Coord, Direction};

        assert_eq!(None, next(&Coord { x: 2, y: 4 }, &Direction::North));
        assert_eq!(None, next(&Coord { x: 4, y: 2 }, &Direction::East));
        assert_eq!(None, next(&Coord { x: 0, y: 2 }, &Direction::West));
        assert_eq!(None, next(&Coord { x: 2, y: 0 }, &Direction::South));
    }

    #[test]
    fn test_step() {
        use heapless::Vec;
        use snakebit::{step, Coord, Direction, GameState};

        let mut state = GameState {
            snake: Vec::from_slice(&[Coord { x: 2, y: 1 }, Coord { x: 2, y: 0 }]).unwrap(),
            dir: Direction::North,
        };

        assert!(step(&mut state));

        assert_eq!(
            Vec::from_slice(&[Coord { x: 2, y: 2 }, Coord { x: 2, y: 1 }]),
            Ok(state.snake)
        );
        assert_eq!(Direction::North, state.dir);
    }

    #[test]
    fn test_step_outside() {
        use heapless::Vec;
        use snakebit::{step, Coord, Direction, GameState};

        let mut state = GameState {
            snake: Vec::from_slice(&[Coord { x: 2, y: 4 }, Coord { x: 2, y: 3 }]).unwrap(),
            dir: Direction::North,
        };

        assert!(!step(&mut state));
    }

    #[test]
    fn test_step_just_behind() {
        use heapless::Vec;
        use snakebit::{step, Coord, Direction, GameState};

        /*
        h = head
        t = tail
        * = body
          (y)
          1  h*
          0  t*
            012 (x)
        */
        let mut state = GameState {
            snake: Vec::from_slice(&[Coord { x: 1, y: 1 }, Coord { x: 2, y: 1 }, Coord { x: 2, y: 0 }, Coord { x: 1, y: 0 }]).unwrap(),
            dir: Direction::South,
        };

        assert!(step(&mut state));
    }

    #[test]
    fn test_step_over() {
        use heapless::Vec;
        use snakebit::{step, Coord, Direction, GameState};

        /*
        h = head
        t = tail
        * = body
          (y)
          1  h*
          0 t**
            012 (x)
        */
        let mut state = GameState {
            snake: Vec::from_slice(&[Coord { x: 1, y: 1 }, Coord { x: 2, y: 1 }, Coord { x: 2, y: 0 }, Coord { x: 1, y: 0 }, Coord { x: 0, y: 0 }]).unwrap(),
            dir: Direction::South,
        };

        assert!(!step(&mut state));
    }

    #[test]
    fn test_render_1() {
        use tiny_led_matrix::Render;
        use snakebit::{Coord,render};
        let result = render(&[Coord { x: 0, y: 0 }]);
        assert_eq!(9, result.brightness_at(0,0));
        assert_eq!(0, result.brightness_at(1,0));
        assert_eq!(0, result.brightness_at(0,1));
    }

    #[test]
    fn test_render_2() {
        use tiny_led_matrix::Render;
        use snakebit::{Coord,render};
        let result = render(&[Coord { x: 0, y: 3 }, Coord { x: 0, y: 4 }]);
        assert_eq!(9, result.brightness_at(0,3));
        assert_eq!(9, result.brightness_at(0,4));
    }

    #[test]
    fn test_turn() {
        use crate::a::state_with_dir;
        use snakebit::{turn_left, turn_right, Direction};
        let mut state = state_with_dir(Direction::North);

        turn_left(&mut state);
        assert_eq!(state.dir, Direction::West);

        turn_left(&mut state);
        assert_eq!(state.dir, Direction::South);

        turn_left(&mut state);
        assert_eq!(state.dir, Direction::East);

        turn_left(&mut state);
        assert_eq!(state.dir, Direction::North);

        turn_right(&mut state);
        assert_eq!(state.dir, Direction::East);

        turn_right(&mut state);
        assert_eq!(state.dir, Direction::South);

        turn_right(&mut state);
        assert_eq!(state.dir, Direction::West);

        turn_right(&mut state);
        assert_eq!(state.dir, Direction::North);
    }
}
