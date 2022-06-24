#[allow(dead_code)]
#[allow(unused_variables)]

pub type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: Vec<Position>,
    pub direction: Direction,
    pub food: Position,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: vec![((width / 2).max(0), (height / 2).max(0))],
            direction: Direction::South,
            food: (2.min(width - 1), height / 2),
        }
    }
}

fn check_range(value: usize, min: usize, max: usize) -> bool {
    return value > min && value < max;
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
        for _ in 1..100 {
            let game = SnakeGame::new(WIDTH, HEIGHT);

            assert!(check_range(game.food.0, 0, WIDTH));
            assert!(check_range(game.food.1, 0, HEIGHT));
        }
    }

    #[test]
    fn should_have_snake_in_bounds() {
        for _ in 1..100 {
            let game = SnakeGame::new(WIDTH, HEIGHT);
            let head = game.snake.first().unwrap();

            assert!(check_range(head.0, 0, WIDTH));
            assert!(check_range(head.1, 0, HEIGHT));
        }
    }
}
