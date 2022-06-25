use std::collections::VecDeque;

#[allow(dead_code)]
#[allow(unused_variables)]

pub type Position = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl std::ops::Not for Direction {
    // Direction will use negation to denote opposite directions
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>,
    pub direction: Direction,
    pub food: Position,
    pub lost: bool,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width / 2).max(0), (height / 2))].into_iter().collect(),
            direction: Direction::South,
            food: (2.min(width - 1), height / 2),
            lost: false,
        }
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
        if let Some(&(x, y)) = self.snake.front() {
            let head: Position;
            match self.direction {
                Direction::North => head = (x, y + 1),
                Direction::South => head = (x, y - 1),
                Direction::West => head = (x - 1, y),
                Direction::East => head = (x + 1, y),
            }

            self.snake.push_front(head);
            self.snake.pop_back();
            self.check_snake_condition();
        }
    }

    fn check_snake_condition(&mut self) {
        if let Some(&(x, y)) = self.snake.front() {
            if x == self.width || x == 0 || y == self.height || y == 0 {
                self.lost = true;
                return;
            }

            let head = (x, y);

            self.lost = self
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

pub fn check_range(value: usize, min: usize, max: usize) -> bool {
    value >= min && value <= max
}

#[allow(dead_code)]
#[cfg(test)]
mod snake {
    use super::*;
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    #[test]
    fn should_construct_with_right_variables() {
        let game = SnakeGame::new(WIDTH, HEIGHT);

        assert_eq!(game.direction, Direction::South);
        assert_eq!(game.width, 10);
        assert_eq!(game.height, 10);
    }

    #[test]
    fn should_have_food_in_bounds() {
        for _ in 1..=10000 {
            let game = SnakeGame::new(WIDTH, HEIGHT);

            assert!(check_range(game.food.0, 0, WIDTH));
            assert!(check_range(game.food.1, 0, HEIGHT));
        }
    }

    #[test]
    fn should_have_snake_in_bounds() {
        for _ in 1..=10000 {
            let game = SnakeGame::new(WIDTH, HEIGHT);
            let head = game.snake.first().unwrap();

            assert!(check_range(head.0, 0, WIDTH));
            assert!(check_range(head.1, 0, HEIGHT));
        }
    }
}

#[cfg(test)]
mod direction {
    use super::*;

    #[test]
    fn should_invert_north_to_only_south() {
        assert_eq!(!Direction::North, Direction::South);
        assert_ne!(!Direction::North, Direction::North);
        assert_ne!(!Direction::North, Direction::West);
        assert_ne!(!Direction::North, Direction::East);
    }

    #[test]
    fn should_invert_south_to_only_north() {
        assert_eq!(!Direction::South, Direction::North);
        assert_ne!(!Direction::South, Direction::South);
        assert_ne!(!Direction::South, Direction::West);
        assert_ne!(!Direction::South, Direction::East);
    }

    #[test]
    fn should_invert_west_to_only_east() {
        assert_eq!(!Direction::West, Direction::East);
        assert_ne!(!Direction::West, Direction::North);
        assert_ne!(!Direction::West, Direction::South);
        assert_ne!(!Direction::West, Direction::West);
    }

    #[test]
    fn should_invert_east_to_only_west() {
        assert_eq!(!Direction::East, Direction::West);
        assert_ne!(!Direction::East, Direction::North);
        assert_ne!(!Direction::East, Direction::South);
        assert_ne!(!Direction::East, Direction::East);
    }
}
