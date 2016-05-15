mod area;
mod color;
mod coords;
mod direction;
mod grid;
mod media;
mod movement;
mod region;
mod settings;
mod style;

pub use self::area::Area;
pub use self::color::Color;
pub use self::coords::Coords;
pub use self::direction::Direction;
pub use self::grid::{ResizeRule, SaveGrid, SplitKind};
pub use self::media::{MediaAlignment, MediaPosition};
pub use self::movement::Movement;
pub use self::region::Region;
pub use self::settings::{BufferSettings, EchoSettings, InputSettings};
pub use self::style::{Style, ConfigStyle, CodeGroup};

pub trait Argument: Copy + Eq {
    fn from_nums<T>(T, Option<Self>) -> Option<Self> where T: Iterator<Item=u64>;
    fn encode(&self) -> String;
    fn decode(args: Option<&str>, default: Option<Self>) -> Option<Self> {
        let iter = args.iter().flat_map(|s| s.split('.')).flat_map(|s| u64::from_str_radix(s, 16));
        Self::from_nums(iter, default)
    }
}

impl Argument for bool {
    fn from_nums<T>(mut args: T, default: Option<bool>) -> Option<bool>
    where T: Iterator<Item=u64> {
        args.next().map_or(default, |n| match n {
            0   => Some(false),
            1   => Some(true),
            _   => default,
        })
    }

    fn encode(&self) -> String {
        if *self { String::from("1") } else { String::from("0") }
    }
}

impl Argument for u32 {
    fn from_nums<T>(mut args: T, default: Option<u32>) -> Option<u32>
    where T: Iterator<Item=u64> {
        args.next().map(|n| n as u32).or(default)
    }

    fn encode(&self) -> String {
        format!("{:x}", self)
    }
}

impl Argument for u64 {
    fn from_nums<T>(mut args: T, default: Option<u64>) -> Option<u64>
    where T: Iterator<Item=u64> {
        args.next().or(default)
    }

    fn encode(&self) -> String {
        format!("{:x}", self)
    }
}

#[cfg(test)]
mod tests {

    use args::*;
    use args::Area::*;
    use args::Direction::*;
    use args::InputSettings::*;
    use args::Movement::*;
    use args::MediaAlignment::*;
    use args::MediaPosition::*;
    use args::Style::*;

    static AREA_TESTS: &'static [(Area, &'static str)] = &[
        (CursorCell, "1"),
        (CursorRow, "2"),
        (CursorColumn, "3"),
        (CursorTo(To(Up, 2, false)), "4.2.1.2.0"),
        (CursorBound(Coords { x: 0, y: 0 }), "5.0.0"),
        (WholeScreen, "6"),
        (Bound(Region { left: 0, top: 0, right: 0x100, bottom: 0x100 }), "6.0.0.100.100"),
        (Rows(0xff, 0xfff), "7.ff.fff"),
        (Columns(0, 0x10), "8.0.10"),
        (BelowCursor(true), "9.1"),
    ];

    static MOVEMENT_TESTS: &'static [(Movement, &'static str)] = &[
        (Position(Coords { x: 0, y: 0 }), "1.0.0"),
        (To(Up, 0x100, false), "2.1.100.0"),
        (ToEdge(Up), "3.1"),
        (To(Down, 0x1b, false), "2.2.1b.0"),
        (ToEdge(Down), "3.2"),
        (To(Left, 2, false), "2.3.2.0"),
        (ToEdge(Left), "3.3"),
        (To(Right, 1, true), "2.4.1.1"),
        (ToEdge(Right), "3.4"),
        (IndexTo(Up, 1), "4.1.1"),
        (IndexTo(Down, 2), "4.2.2"),
        (IndexTo(Left, 0xfff), "4.3.fff"),
        (IndexTo(Right, 0x10), "4.4.10"),
        (Tab(Left, 1, false), "5.3.1.0"),
        (Tab(Right, 6, false), "5.4.6.0"),
        (PreviousLine(1), "6.1.1"),
        (NextLine(0xf), "6.f"),
        (Column(0), "7.0"),
        (Row(1), "8.1"),
        (ToBeginning, "9.1"),
        (ToEnd, "9"),
    ];

    static STYLE_TESTS: &'static [(Style, &'static str)] = &[
        (Underline(1), "1.1"),
        (Bold(true), "2.1"),
        (Italic(false), "3.0"),
        (Blink(false), "4.0"),
        (InvertColors(false), "5.0"),
        (Strikethrough(true), "6.1"),
        (Opacity(0x40), "7.40"),
        (FgColor(Color::True(0, 1, 0x19)), "8.2.0.1.19"),
        (BgColor(Color::True(0xff, 0xfe, 0xf)), "9.2.ff.fe.f"),
        (FgColor(Color::Default), "8.0"),
        (FgColor(Color::Palette(7)), "8.1.7"),
        (BgColor(Color::Default), "9.0"),
        (BgColor(Color::Palette(0xf)), "9.1.f"),
    ];

    fn run_test<T: Argument + ::std::fmt::Debug>(strings: &str, args: &[T]) {
        for (s, &arg) in strings.split(";").zip(args) {
            assert_eq!(T::decode(Some(s), None), Some(arg));
            assert_eq!(arg.encode(), s);
        }
    }

    #[test]
    fn area_argument() {
        for &(area, arg) in AREA_TESTS {
            assert_eq!(Area::decode(Some(arg), None), Some(area));
            assert_eq!(area.encode(), arg);
        }
    }

    #[test]
    fn bool_argument() {
        run_test("0;1", &[false, true]);
    }

    #[test]
    fn color_argument() {
        run_test("2.0.1.2;2.3.4.5", &[Color::True(0,1,2), Color::True(3,4,5)]);
    }

    #[test]
    fn coords_argument() {
        run_test("1.2;3.4", &[Coords{x:1, y:2}, Coords{x:3, y:4}]);
    }

    #[test]
    fn direction_argument() {
        run_test("1;2;3;4", &[Up, Down, Left, Right]);
    }

    #[test]
    fn input_settings_argument() {
        run_test("1;2", &[Ansi(false), Notty(())]);
    }

    #[test]
    fn media_align_argument() {
        run_test("1;2;3", &[LeftTop, Center, RightBottom]);
    }


    #[test]
    fn media_pos_argument() {
        run_test("1.2.3;4;5", &[Display(Center, RightBottom), Stretch, Tile]);
    }

    #[test]
    fn movement_argument() {
        for &(movement, arg) in MOVEMENT_TESTS {
            assert_eq!(Movement::decode(Some(arg), None), Some(movement));
            assert_eq!(movement.encode(), arg);
        }
    }

    #[test]
    fn region_argument() {
        run_test("0.1.2.3", &[Region::new(0,1,2,3)]);
    }

    #[test]
    fn style_argument() {
        for &(style, arg) in STYLE_TESTS {
            assert_eq!(Style::decode(Some(arg), None), Some(style));
            assert_eq!(&style.encode(), arg);
        }
    }

}
