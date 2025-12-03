pub mod solution;
pub mod transform;

pub use solution::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn add(&self, x: &mut usize, y: &mut usize) {
        match self {
            Self::North => *y -= 1,
            Self::South => *y += 1,
            Self::East => *x += 1,
            Self::West => *x -= 1,
        }
    }

    pub fn offset(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Self::North => (x, y - 1),
            Self::South => (x, y + 1),
            Self::East => (x + 1, y),
            Self::West => (x - 1, y),
        }
    }

    pub fn diff(&self, other: &Self) -> u8 {
        ((*self as i8) - (*other as i8)).abs() as u8
    }
}
