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

#[cfg(test)]
mod snake {
    use super::*;

    #[test]
    fn output() {
        println!("{:?}", SnakeGame::new(10, 10));
    }

    #[test]
    fn should_construct_with_right_variables() {
        let game = SnakeGame::new(10, 10);

        assert_eq!(game.direction, Direction::South);
        assert_eq!(game.width, 10);
        assert_eq!(game.height, 10);
    }
}
