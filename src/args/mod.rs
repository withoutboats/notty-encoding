pub mod argument;
mod movement;
mod region;

pub use self::argument::Argument;
pub use self::movement::Movement;
pub use self::region::Region;

use std::fmt;
use std::ops::Index;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferSettings {
    pub eol1: u8,
    pub eol2: u8,
    pub eof: u8,
    pub intr: u8,
    pub quit: u8,
    pub susp: u8,
}

impl BufferSettings {

    pub fn eof(&self, c: char) -> bool {
        if let '\0'...'\x7f' = c {
            c as u8 == self.eof
        } else { false }
    }

    pub fn eol(&self, c: char) -> bool {
        if let '\0'...'\x7f' = c {
            c as u8 == self.eol1 || c as u8 == self.eol2 || c as u8 == self.eof
        } else { false }
    }

    pub fn signal(&self, c: char) -> bool {
        if let '\0'...'\x7f' = c {
            c as u8 == self.intr || c as u8 == self.quit || c as u8 == self.susp
        } else { false }
    }
}

/// A 24-bit rgb color sequence.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub enum PaletteError {
    IncorrectSize(i32),
}

impl fmt::Display for PaletteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PaletteError::IncorrectSize(n) if n > 0 => {
                write!(f, "{} too many 'Color' specified to define a Palette.",
                       n.abs())
            },
            PaletteError::IncorrectSize(n) if n < 0 => {
                write!(f, "{} too few 'Color' specified to define a Palette.",
                       n.abs())
            },
            PaletteError::IncorrectSize(_) => {
                write!(f, "Unspecified Palette size error.",)
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new_from_slice(colors: &[Color]) -> Result<Palette, PaletteError> {
        let slice_length = colors.len();
        if slice_length - 256 != 0 {
            Err(PaletteError::IncorrectSize(slice_length as i32 - 256))
        } else {
            let mut v = Vec::with_capacity(256);
            for color in colors {
                v.push(color.clone());
            }
            Ok(Palette{colors: v.clone()})
        }
    }
}

impl Index<usize> for Palette {
    type Output = Color;

    fn index<'a>(&'a self, index: usize) -> &'a Color {
        &self.colors[index]
    }
}

/// A corodinate pair.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Coords {
    pub x: u32,
    pub y: u32,
}

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EchoSettings {
    pub lerase: u8,
    pub lnext: u8,
    pub werase: u8,
}

/// The mode the input processor is in.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InputSettings {
    /// Ansi-compatible mode, boolean determines of "application" mode or not.
    Ansi(bool),
    /// Notty mode.
    Notty(()),
    LineBufferEcho(EchoSettings, BufferSettings),
    ScreenEcho(EchoSettings),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MediaAlignment {
    LeftTop, Center, RightBottom
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MediaPosition {
    Display(MediaAlignment, MediaAlignment),
    Fill,
    Fit,
    Stretch,
    Tile
}

impl Default for MediaPosition {
    fn default() -> MediaPosition {
        MediaPosition::Display(MediaAlignment::LeftTop, MediaAlignment::LeftTop)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResizeRule {
    Percentage,
    MaxLeftTop,
    MaxRightBottom,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SaveGrid {
    Left, Right
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SplitKind {
    Horizontal(u32),
    Vertical(u32),
}

/// Set rich text styles. Booleans represent on or off.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Style {
    /// Field is number of underlines (between 0 and 2).
    Underline(u8),
    Bold(bool),
    Italic(bool),
    Blink(bool),
    InvertColors(bool),
    Strikethrough(bool),
    Opacity(u8),
    FgColor(Color),
    FgColorCfg(Option<u8>),
    BgColor(Color),
    BgColorCfg(Option<u8>),
}
