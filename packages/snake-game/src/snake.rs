use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::board_state::{BoardState, IntoBoardState};
use crate::random::random_range;
use crate::utils::Direction;

pub type Position = (usize, usize);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>,
    pub direction: Direction,
    pub food: Position,
    pub has_lost: bool,
    pub is_active: bool,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width / 2).max(0), (height / 2))].into_iter().collect(),
            direction: Direction::West,
            food: (2.min(width - 1), height / 2),
            has_lost: false,
            is_active: false,
        }
    }

    pub fn start(&mut self) {
        self.is_active = true;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.direction == direction || self.direction == !direction {
            return;
        } else {
            self.direction = direction;
            return;
        }
    }

    pub fn tick(&mut self) {
        if self.has_lost || !self.is_active {
            return;
        }

        if let Some(&(x, y)) = self.snake.front() {
            let head: Position;
            match self.direction {
                Direction::North => head = (x, y + 1),
                Direction::South => head = (x, y - 1),
                Direction::West => head = (x - 1, y),
                Direction::East => head = (x + 1, y),
            }

            self.snake.push_front(head);
            if head != self.food {
                self.snake.pop_back();
            } else {
                let open_positions = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                    .filter(|&pos| !self.snake.contains(&pos))
                    .collect::<Vec<Position>>();

                self.food = open_positions
                    .get(random_range(0, open_positions.len()))
                    .unwrap()
                    .to_owned();
            }
            self.check_snake_condition();
        }
    }

    fn check_snake_condition(&mut self) {
        if let Some(&(x, y)) = self.snake.front() {
            if x == self.width || x == 0 || y == self.height || y == 0 {
                self.has_lost = true;
                return;
            }

            let head = (x, y);

            self.has_lost = self
                .snake
                .iter()
                .filter(|&&x| x == head)
                .cloned()
                .collect::<VecDeque<Position>>()
                .len()
                .gt(&1);
        }
    }
}

impl IntoBoardState for SnakeGame {
    fn convert_to_board_state(&mut self) -> BoardState {
        todo!()
    }
}
#[allow(dead_code)]
#[cfg(test)]
mod snake {
    use super::*;
    use crate::utils::check_range;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    #[test]
    fn should_construct_with_right_variables() {
        let game = SnakeGame::new(WIDTH, HEIGHT);

        assert_eq!(game.direction, Direction::West);
        assert_eq!(game.width, 10);
        assert_eq!(game.height, 10);
    }

    #[test]
    fn should_have_food_in_bounds() {
        for _ in 1..=10000 {
            let game = SnakeGame::new(WIDTH, HEIGHT);
            let (x, y) = game.food;

            assert!(check_range(x, 0, WIDTH));
            assert!(check_range(y, 0, HEIGHT));
        }
    }

    #[test]
    fn should_have_snake_in_bounds() {
        for _ in 1..=10000 {
            let game = SnakeGame::new(WIDTH, HEIGHT);
            let &(x, y) = game.snake.front().unwrap();

            assert!(check_range(x, 0, WIDTH));
            assert!(check_range(y, 0, HEIGHT));
        }
    }

    #[test]
    fn should_flag_as_has_lost_if_head_hits_bounds() {
        let mut direction: Direction;
        let mut starting_position: Position;
        // @note There's an nicer way to do this with some enum iterator macro, but
        //       I don't want to install a crate _just_ for this.
        for d in 0..=3 {
            match d {
                0 => {
                    direction = Direction::North;
                    starting_position = (5, 4);
                }
                1 => {
                    direction = Direction::South;
                    starting_position = (5, 6);
                }
                2 => {
                    direction = Direction::West;
                    starting_position = (6, 5)
                }
                3 => {
                    direction = Direction::East;
                    starting_position = (4, 5);
                }
                _ => panic!("Should never hit here. There are only four directions."),
            }

            let mut game = SnakeGame {
                width: WIDTH,
                height: HEIGHT,
                snake: vec![starting_position].into(),
                direction,
                food: (0_usize, 0_usize),
                has_lost: false,
                is_active: true,
            };

            for i in 0..=5 {
                game.tick();
                if i != 5 {
                    assert!(!game.has_lost);
                }
            }

            assert!(game.has_lost);
        }
    }

    #[test]
    fn should_flas_as_has_lost_if_head_hits_snake() {
        // We're just going to draw a square with points
        let positions: VecDeque<Position> = vec![
            (5, 5), // head
            (5, 6),
            (6, 6),
            (7, 6),
            (7, 5),
            (7, 4),
            (7, 3),
            (6, 3),
            (5, 3), // head should hit this point after two ticks
            (4, 3),
            (3, 3),
        ]
        .into();

        let mut game = SnakeGame {
            width: 1000,
            height: 1000,
            snake: positions,
            direction: Direction::South,
            food: (0_usize, 0_usize),
            has_lost: false,
            is_active: true,
        };

        game.tick();
        assert!(!game.has_lost); // first movement down

        game.tick();
        assert!(game.has_lost); // second movement, hitting 11th node on (5, 3)
    }
}
