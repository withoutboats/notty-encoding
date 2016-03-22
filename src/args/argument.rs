use args::*;
use args::Area::*;
use args::Direction::*;
use args::InputSettings::*;
use args::Movement::*;
use args::MediaAlignment::*;
use args::MediaPosition::*;
use args::Style::*;

pub trait Argument: Copy + Eq {
    fn from_nums<T>(T, Option<Self>) -> Option<Self> where T: Iterator<Item=u64>;
    fn encode(&self) -> String;
    fn decode(args: Option<&str>, default: Option<Self>) -> Option<Self> {
        let iter = args.iter().flat_map(|s| s.split('.')).flat_map(|s| u64::from_str_radix(s, 16));
        Self::from_nums(iter, default)
    }
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

impl Argument for BufferSettings {

    fn from_nums<T>(mut args: T, default: Option<BufferSettings>) -> Option<BufferSettings>
    where T: Iterator<Item=u64> {
        let intr = match args.next() { Some(n) => n as u8, None => return default };
        let quit = match args.next() { Some(n) => n as u8, None => return default };
        let susp = match args.next() { Some(n) => n as u8, None => return default };
        let eol1 = match args.next() { Some(n) => n as u8, None => return default };
        let eol2 = match args.next() { Some(n) => n as u8, None => return default };
        let eof  = match args.next() { Some(n) => n as u8, None => return default };
        Some(BufferSettings {
            eol1: eol1,
            eol2: eol2,
            eof: eof,
            intr: intr,
            quit: quit,
            susp: susp,
        })
    }

    fn encode(&self) -> String {
        format!("{:x}.{:x}.{:x}.{:x}.{:x}.{:x}", self.intr, self.quit, self.susp, self.eol1,
                self.eol2, self.eof)
    }

}

impl Argument for Color {

    fn from_nums<T>(mut args: T, default: Option<Color>) -> Option<Color>
    where T: Iterator<Item=u64> {
        match (args.next(), args.next(), args.next()) {
            (Some(r), Some(g), Some(b)) => Some(Color(r as u8, g as u8, b as u8)),
            _                           => default,
        }
    }

    fn encode(&self) -> String {
        format!("{:x}.{:x}.{:x}", self.0, self.1, self.2)
    }
}

impl Argument for Coords {

    fn from_nums<T>(mut args: T, default: Option<Coords>) -> Option<Coords>
    where T: Iterator<Item=u64> {
        match (args.next(), args.next()) {
            (Some(x), Some(y))  => Some(Coords { x: x as u32, y: y as u32 }),
            _                   => default,
        }
    }

    fn encode(&self) -> String {
        format!("{:x}.{:x}", self.x, self.y)
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

impl Argument for EchoSettings {

    fn from_nums<T>(mut args: T, default: Option<EchoSettings>) -> Option<EchoSettings>
    where T: Iterator<Item=u64> {
        let lerase = match args.next() { Some(n) => n as u8, None => return default };
        let lnext = match args.next() { Some(n) => n as u8, None => return default };
        let werase = match args.next() { Some(n) => n as u8, None => return default };
        Some(EchoSettings {
            lerase: lerase,
            lnext: lnext,
            werase: werase,
        })
    }

    fn encode(&self) -> String {
        format!("{:x}.{:x}.{:x}", self.lerase, self.lnext, self.werase)
    }
}

impl Argument for InputSettings {
    
    fn from_nums<T>(mut args: T, default: Option<InputSettings>) -> Option<InputSettings>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(1) => Some(Ansi(false)),
            Some(2) => Some(Notty(())),
            Some(3) => EchoSettings::from_nums(args.by_ref(), None).and_then(|echo| {
                BufferSettings::from_nums(args, None).map(|buffer| LineBufferEcho(echo, buffer))
            }),
            Some(4) => EchoSettings::from_nums(args, None).map(ScreenEcho),
            _       => default,
        }
    }

    fn encode(&self) -> String {
        match *self {
            Ansi(_)                         => String::from("1"),
            Notty(_)                        => String::from("2"),
            LineBufferEcho(echo, buffer)    => format!("3.{}.{}", echo.encode(), buffer.encode()),
            ScreenEcho(echo)                => format!("4.{}", echo.encode()),
        }
    }

}

impl Argument for MediaAlignment {

    fn from_nums<T>(mut args: T, default: Option<MediaAlignment>) -> Option<MediaAlignment>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(1) => Some(LeftTop),
            Some(2) => Some(Center),
            Some(3) => Some(RightBottom),
            _       => default,
        }
    }

    fn encode(&self) -> String {
        match *self {
            LeftTop     => String::from("1"),
            Center      => String::from("2"),
            RightBottom => String::from("3"),
        }
    }

}

impl Argument for MediaPosition {

    fn from_nums<T>(mut args: T, default: Option<MediaPosition>) -> Option<MediaPosition>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(1) => {
                let horizontal = MediaAlignment::from_nums(args.by_ref(), Some(LeftTop)).unwrap();
                let vertical = MediaAlignment::from_nums(args, Some(RightBottom)).unwrap();
                Some(Display(horizontal, vertical))
            }
            Some(2) => Some(Fill),
            Some(3) => Some(Fit),
            Some(4) => Some(Stretch),
            Some(5) => Some(Tile),
            _       => default
        }
    }

    fn encode(&self) -> String {
        match *self {
            Display(hor, ver)   => format!("1.{}.{}", hor.encode(), ver.encode()),
            Fill                => String::from("2"),
            Fit                 => String::from("3"),
            Stretch             => String::from("4"),
            Tile                => String::from("5"),
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

impl Argument for Region {

    fn from_nums<T>(mut args: T, default: Option<Region>) -> Option<Region>
    where T: Iterator<Item=u64> {
        match (args.next(), args.next(), args.next(), args.next()) {
            (Some(l), Some(t), Some(r), Some(b)) => Some(Region::new(l as u32, t as u32,
                                                                     r as u32, b as u32)),
            _                                    => default
        }
    }

    fn encode(&self) -> String {
        format!("{:x}.{:x}.{:x}.{:x}", self.left, self.top, self.right, self.bottom)
    }

}

impl Argument for ResizeRule {

    fn from_nums<T>(mut args: T, default: Option<ResizeRule>) -> Option<ResizeRule>
    where T: Iterator<Item=u64> {
        args.next().and_then(|arg| match arg {
            0 => Some(ResizeRule::Percentage),
            1 => Some(ResizeRule::MaxLeftTop),
            2 => Some(ResizeRule::MaxRightBottom),
            _ => None
        }).or(default)
    }

    fn encode(&self) -> String {
        match *self {
            ResizeRule::Percentage      => String::from("0"),
            ResizeRule::MaxLeftTop      => String::from("1"),
            ResizeRule::MaxRightBottom  => String::from("2"),
        }
    }

}

impl Argument for SaveGrid {

    fn from_nums<T>(mut args: T, default: Option<SaveGrid>) -> Option<SaveGrid>
    where T: Iterator<Item=u64> {
        args.next().and_then(|arg| match arg {
            0 => Some(SaveGrid::Left),
            1 => Some(SaveGrid::Right),
            _ => None
        }).or(default)
    }

    fn encode(&self) -> String {
        match *self {
            SaveGrid::Left  => String::from("0"),
            SaveGrid::Right => String::from("1"),
        }
    }

}

impl Argument for SplitKind {
    fn from_nums<T>(mut args: T, default: Option<SplitKind>) -> Option<SplitKind>
    where T: Iterator<Item=u64> {
        match (args.next(), args.next()) {
            (Some(0), Some(n))  => Some(SplitKind::Horizontal(n as u32)),
            (Some(1), Some(n))  => Some(SplitKind::Vertical(n as u32)),
            (None, None)        => default,
            _                   => None,
        }
    }

    fn encode(&self) -> String {
        match *self {
            SplitKind::Horizontal(n)    => format!("0.{:x}", n),
            SplitKind::Vertical(n)      => format!("1.{:x}", n),
        }
    }
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
            Some(0xa)   => Some(FgColorCfg(args.next().map(|x| x as u8))),
            Some(0xb)   => Some(BgColorCfg(args.next().map(|x| x as u8))),
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
            FgColorCfg(None)    => format!("a"),
            FgColorCfg(Some(n)) => format!("a.{:x}", n),
            BgColorCfg(None)    => format!("b"),
            BgColorCfg(Some(n)) => format!("b.{:x}", n),
        }
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
        (Style::Underline(1), "1.1"),
        (Style::Bold(true), "2.1"),
        (Style::Italic(false), "3.0"),
        (Style::Blink(false), "4.0"),
        (Style::InvertColors(false), "5.0"),
        (Style::Strikethrough(true), "6.1"),
        (Style::Opacity(0x40), "7.40"),
        (Style::FgColor(Color(0, 1, 0x19)), "8.0.1.19"),
        (Style::BgColor(Color(0xff, 0xfe, 0xf)), "9.ff.fe.f"),
        (Style::FgColorCfg(None), "a"),
        (Style::FgColorCfg(Some(7)), "a.7"),
        (Style::BgColorCfg(None), "b"),
        (Style::BgColorCfg(Some(0xf)), "b.f"),
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
        run_test("0.1.2;3.4.5", &[Color(0,1,2), Color(3,4,5)]);
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
