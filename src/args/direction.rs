use super::Argument;
use self::Direction::*;

/// A direction of movement across the grid.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rev(&self) -> Direction {
        match *self {
            Direction::Up       => Direction::Down,
            Direction::Down     => Direction::Up,
            Direction::Left     => Direction::Right,
            Direction::Right    => Direction::Left,
        }
    }
}

impl Argument for Direction {
    fn from_nums<T>(mut args: T, default: Option<Direction>) -> Option<Direction>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(1) => Some(Up),
            Some(2) => Some(Down),
            Some(3) => Some(Left),
            Some(4) => Some(Right),
            _       => default
        }
    }

    fn encode(&self) -> String {
        match *self {
            Up      => String::from("1"),
            Down    => String::from("2"),
            Left    => String::from("3"),
            Right   => String::from("4"),
        }
    }
}
