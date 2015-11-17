use args::{Argument, Direction, Movement};
use cmds::EscCode;

#[derive(Copy, Clone)]
pub struct Move {
    pub movement: Movement,
}

impl Move {
    pub fn new(movement: Movement) -> Move {
        Move { movement: movement }
    }
}

impl EscCode for Move {
    fn opcode() -> &'static str { "18" }
    fn args(&self) -> Vec<String> {
        vec![self.movement.encode()]
    }
}

pub struct ScrollScreen {
    pub dir: Direction,
    pub n: u32,
}

impl ScrollScreen {
    pub fn new(dir: Direction, n: u32) -> ScrollScreen {
        ScrollScreen {
            dir: dir,
            n: n
        }
    }
}

impl EscCode for ScrollScreen {
    fn opcode() -> &'static str { "19" }
    fn args(&self) -> Vec<String> {
        vec![self.dir.encode(), self.n.encode()]
    }
}
