use super::Argument;

/// A corodinate pair.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Coords {
    pub x: u32,
    pub y: u32,
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
