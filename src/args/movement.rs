use std::cmp::Ordering;

use super::{Argument, Coords, Direction};
use super::Direction::*;

use self::Movement::*;

/// Represents a manner in which the cursor can be moved.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Movement {
    Position(Coords),
    To(Direction, u32, bool),
    ToEdge(Direction),
    IndexTo(Direction, u32),
    /// Arguments:
    /// * Direction of the tab character.
    /// * Number of tab stops "long" the movement should be.
    /// * Whether or not the movement should wrap when it reaches the end of the screen.
    Tab(Direction, u32, bool ),
    Column(u32),
    Row(u32),
    PreviousLine(u32),
    NextLine(u32),
    ToBeginning,
    ToEnd,
}

impl Movement {
    /// Returns the direction the cursor would travel in on taking this movement.
    pub fn direction(&self, cursor: Coords) -> Direction {
        match *self {
            To(d, _, _) | ToEdge(d) | IndexTo(d, _) | Tab(d, _, _)  => d,
            ToBeginning                                             => Left,
            ToEnd                                                   => Right,
            PreviousLine(_)                                         => Up,
            NextLine(_)                                             => Down,
            Column(n) if n < cursor.x                               => Left,
            Column(_)                                               => Right,
            Row(n) if n < cursor.y                                  => Up,
            Row(_)                                                  => Down,
            Position(coords)                                        => {
                match coords.y.cmp(&cursor.y) {
                    Ordering::Less                                  => Left,
                    Ordering::Equal if coords.x < cursor.x          => Left,
                    Ordering::Equal                                 => Right,
                    Ordering::Greater                               => Right,
                }
            }
        }
    }

    /// Returns true if this motion can cause the screen to scroll.
    pub fn scrolls(&self) -> bool {
        match *self {
            IndexTo(..) | PreviousLine(_) | NextLine(_) => true,
           _                                            => false,
        }
    }
}

impl Argument for Movement {
    fn from_nums<T>(mut args: T, default: Option<Movement>) -> Option<Movement>
    where T: Iterator<Item=u64> {
        match args.next() {
            // Position
            Some(0x1)   => Coords::from_nums(args, Some(Coords {x: 0, y: 0})).map(Position),
            // To
            Some(0x2)   => {
                let dir = Direction::from_nums(args.by_ref(), Some(Right)).unwrap();
                let n = u32::from_nums(args.by_ref(), Some(1)).unwrap();
                let wrap = bool::from_nums(args, Some(false)).unwrap();
                Some(To(dir, n, wrap))
            }
            // ToEdge
            Some(0x3)   => Direction::from_nums(args, Some(Right)).map(ToEdge),
            // IndexTo
            Some(0x4)   => {
                let dir = Direction::from_nums(args.by_ref(), Some(Right)).unwrap();
                let n = u32::from_nums(args.by_ref(), Some(1)).unwrap();
                Some(IndexTo(dir, n))
            }
            // Tab
            Some(0x5)   => {
                let dir = Direction::from_nums(args.by_ref(), Some(Right)).unwrap();
                let n = u32::from_nums(args.by_ref(), Some(1)).unwrap();
                let wrap = bool::from_nums(args, Some(false)).unwrap();
                Some(Tab(dir, n, wrap))
            }
            // PreviousLine/NextLine
            Some(0x6)   => {
                let n = u32::from_nums(args.by_ref(), Some(1)).unwrap();
                match bool::from_nums(args, Some(false)).unwrap() {
                    true    => Some(PreviousLine(n)),
                    false   => Some(NextLine(n)),
                }
            }
            // Column
            Some(0x7)   => u32::from_nums(args, Some(0)).map(Column),
            // Row
            Some(0x8)   => u32::from_nums(args, Some(0)).map(Row),
            // ToBeginning/ToEnd
            Some(0x9)   => {
                match bool::from_nums(args, Some(false)).unwrap() {
                    true    => Some(ToBeginning),
                    false   => Some(ToEnd),
                }
            }
            _                   => default,
        }
    }

    fn encode(&self) -> String {
        match *self {
            Position(coords)    => format!("1.{}", coords.encode()),
            To(dir, n, wrap)    => format!("2.{}.{:x}.{}", dir.encode(), n, wrap.encode()),
            ToEdge(dir)         => format!("3.{}", dir.encode()),
            IndexTo(dir, n)     => format!("4.{}.{:x}", dir.encode(), n),
            Tab(dir, n, wrap)   => format!("5.{}.{:x}.{}", dir.encode(), n, wrap.encode()),
            PreviousLine(n)     => format!("6.{:x}.1", n),
            NextLine(n)         => format!("6.{:x}", n),
            Column(n)           => format!("7.{:x}", n),
            Row(n)              => format!("8.{:x}", n),
            ToBeginning         => String::from("9.1"),
            ToEnd               => String::from("9"),
        }
    }
}

