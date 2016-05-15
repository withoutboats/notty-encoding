use super::Argument;
use self::MediaAlignment::*;
use self::MediaPosition::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MediaAlignment {
    LeftTop, Center, RightBottom
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MediaPosition {
    Display(MediaAlignment, MediaAlignment),
    Fill,
    Fit,
    Stretch,
    Tile
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

impl Default for MediaPosition {
    fn default() -> MediaPosition {
        MediaPosition::Display(MediaAlignment::LeftTop, MediaAlignment::LeftTop)
    }
}
