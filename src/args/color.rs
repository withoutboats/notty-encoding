use super::Argument;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    /// The default color for this item.
    Default,
    /// An index into the 256-member provided color palette.
    Palette(u8),
    /// A 24-bit rgb color triple.
    True(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Color {
        Color::Default
    }
}

impl Argument for Color {
    fn from_nums<T>(mut args: T, default: Option<Color>) -> Option<Color>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(0) => Some(Color::Default),
            Some(1) => args.next().map(|n| Color::Palette(n as u8)),
            Some(2) => match (args.next(), args.next(), args.next()) {
                (Some(r), Some(g), Some(b)) => Some(Color::True(r as u8, g as u8, b as u8)),
                _                           => None
            },
            _       => default
        }
    }

    fn encode(&self) -> String {
        match *self {
            Color::Default          => String::from("0"),
            Color::Palette(n)       => format!("1.{:x}", n),
            Color::True(r, g, b)    => format!("2.{:x}.{:x}.{:x}", r, g, b)
        }
    }
}
