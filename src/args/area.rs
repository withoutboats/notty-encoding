use super::{Argument, Coords, Movement, Region};
use self::Area::*;

/// An abstractly defined section of the grid.
///
/// Areas can be defined in terms of the current cursor position and the bounds of the grid. They
/// are converted into concrete sections of the screen when commands using Areas are applied.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Area {
    /// The cell the cursor is in.
    CursorCell,
    /// The row the cursor is in.
    CursorRow,
    /// The column the cursor is in.
    CursorColumn,
    /// All cells the cursor would traverse through in performing a movement (including the cell
    /// the cursor is in now, and the cell it would end in).
    CursorTo(Movement),
    /// The rectangle bound in one corner by the cursor position and another by this coordinate.
    CursorBound(Coords),
    /// The entire screen.
    WholeScreen,
    /// A concrete rectangular section of the screen.
    Bound(Region),
    /// The rows between the two parameters, inclusive of the first but not the second.
    Rows(u32, u32),
    /// The columns between the two parameters, inclusive of the first but not the second.
    Columns(u32, u32),
    /// Everything below the row the cursor is in, the boolean determines if this is inclusive of
    /// the cursor or not (inclusive = true).
    BelowCursor(bool),
}

impl Argument for Area {
    fn from_nums<T>(mut args: T, default: Option<Area>) -> Option<Area>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(1) => Some(CursorCell),
            Some(2) => Some(CursorRow),
            Some(3) => Some(CursorColumn),
            Some(4) => Movement::from_nums(args, None).map(CursorTo),
            Some(5) => Coords::from_nums(args, None).map(CursorBound),
            Some(6) => Region::from_nums(args, None).map(Bound).or(Some(WholeScreen)),
            Some(7) => match (args.next(), args.next()) {
                (Some(top), Some(bottom))   => Some(Rows(top as u32, bottom as u32)),
                _                           => Some(WholeScreen),
            },
            Some(8) => match (args.next(), args.next()) {
                (Some(top), Some(bottom))   => Some(Columns(top as u32, bottom as u32)),
                _                           => Some(WholeScreen),
            },
            Some(9) => bool::from_nums(args, Some(true)).map(BelowCursor),
            _       => default,
        }
    }

    fn encode(&self) -> String {
        match *self {
            CursorCell              => String::from("1"),
            CursorRow               => String::from("2"),
            CursorColumn            => String::from("3"),
            CursorTo(mov)           => format!("4.{}", mov.encode()),
            CursorBound(coords)     => format!("5.{}", coords.encode()),
            WholeScreen             => format!("6"),
            Bound(region)           => format!("6.{}", region.encode()),
            Rows(top, bottom)       => format!("7.{:x}.{:x}", top, bottom),
            Columns(left, right)    => format!("8.{:x}.{:x}", left, right),
            BelowCursor(b)          => format!("9.{}", b.encode()),
        }
    }
}
