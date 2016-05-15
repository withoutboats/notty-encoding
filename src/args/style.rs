use super::{Argument, Color};
use self::Style::*;

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
    BgColor(Color),
}

impl Argument for Style {
    fn from_nums<T>(mut args: T, default: Option<Style>) -> Option<Style>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(0x1)   => match args.next() {
                Some(0)         => Some(Underline(0)),
                Some(1) | None  => Some(Underline(1)),
                Some(2)         => Some(Underline(2)),
                _               => None
            },
            Some(0x2)   => bool::from_nums(args, Some(true)).map(Bold),
            Some(0x3)   => bool::from_nums(args, Some(true)).map(Italic),
            Some(0x4)   => bool::from_nums(args, Some(true)).map(Blink),
            Some(0x5)   => bool::from_nums(args, Some(true)).map(InvertColors),
            Some(0x6)   => bool::from_nums(args, Some(true)).map(Strikethrough),
            Some(0x7)   => Some(Opacity(args.next().unwrap_or(0xff) as u8)),
            Some(0x8)   => Color::from_nums(args, None).map(FgColor),
            Some(0x9)   => Color::from_nums(args, None).map(BgColor),
            _           => default
        }
    }

    fn encode(&self) -> String {
        match *self {
            Underline(n)        => format!("1.{:x}", n),
            Bold(flag)          => format!("2.{}", flag.encode()),
            Italic(flag)        => format!("3.{}", flag.encode()),
            Blink(flag)         => format!("4.{}", flag.encode()),
            InvertColors(flag)  => format!("5.{}", flag.encode()),
            Strikethrough(flag) => format!("6.{}", flag.encode()),
            Opacity(n)          => format!("7.{:x}", n),
            FgColor(color)      => format!("8.{}", color.encode()),
            BgColor(color)      => format!("9.{}", color.encode()),
        }
    }
}
