use super::{Argument, Color};
use self::Style::*;
use self::CodeGroup::*;

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
    Configured(ConfigStyle),
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
            Some(0xa)   => ConfigStyle::from_nums(args, None).map(Configured),
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
            Configured(style)   => format!("a.{}", style.encode()),
        }
    }
}

pub const DEFAULT_CONFIG_STYLE: ConfigStyle = ConfigStyle::Plain;

/// Some means of identifying a preconfigured style provided by the user.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ConfigStyle {
    Plain,
    CodeGroup(CodeGroup),
}

impl Argument for ConfigStyle {
    fn from_nums<T>(mut args: T, default: Option<ConfigStyle>) -> Option<ConfigStyle>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(1) => Some(ConfigStyle::Plain),
            Some(2) => CodeGroup::from_nums(args, None).map(ConfigStyle::CodeGroup),
            _       => default
        }
    }

    fn encode(&self) -> String {
        match *self {
            ConfigStyle::Plain              => String::from("1"),
            ConfigStyle::CodeGroup(group)   => format!("2.{}", group.encode()),
        }
    }
}

impl Default for ConfigStyle {
    fn default() -> ConfigStyle {
        DEFAULT_CONFIG_STYLE
    }
}

/// A kind of text found in source code, for syntax highlighting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CodeGroup {
    Comment,
    Documentation,
    Error,
    Identifier,
    Keyword,
    Literal,
    Macro,
    Special,
    Type,
    Todo,
}

impl Argument for CodeGroup {
    fn from_nums<T>(mut args: T, default: Option<CodeGroup>) -> Option<CodeGroup>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(1)     => Some(Keyword),
            Some(2)     => Some(Identifier),
            Some(3)     => Some(Type),
            Some(4)     => Some(Literal),
            Some(5)     => Some(Macro),
            Some(6)     => Some(Comment),
            Some(7)     => Some(Documentation),
            Some(8)     => Some(Error),
            Some(9)     => Some(Todo),
            Some(10)    => Some(Special),
            _           => default,
        }
    }

    fn encode(&self) -> String {
        match *self {
            Keyword         => String::from("1"),
            Identifier      => String::from("2"),
            Type            => String::from("3"),
            Literal         => String::from("4"),
            Macro           => String::from("5"),
            Comment         => String::from("6"),
            Documentation   => String::from("7"),
            Error           => String::from("8"),
            Todo            => String::from("9"),
            Special         => String::from("a"),
        }
    }
}
