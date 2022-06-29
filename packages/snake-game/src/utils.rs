use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

pub fn check_range(value: usize, min: usize, max: usize) -> bool {
    value >= min && value <= max
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
