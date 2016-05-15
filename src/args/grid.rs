use super::Argument;

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
