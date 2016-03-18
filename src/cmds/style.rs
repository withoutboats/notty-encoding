use args::{Argument, Area, Style};
use cmds::EscCode;

pub struct SetTextStyle(pub Style);

impl EscCode for SetTextStyle {
    const OPCODE: u16 = 0x30;
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}

pub struct DefaultTextStyle;

impl EscCode for DefaultTextStyle {
    const OPCODE: u16 = 0x30;
}

pub struct SetCursorStyle(pub Style);

impl EscCode for SetCursorStyle {
    const OPCODE: u16 = 0x31;
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}

pub struct DefaultCursorStyle;

impl EscCode for DefaultCursorStyle {
    const OPCODE: u16 = 0x31;
}

pub struct SetStyleInArea(pub Area, pub Style);

impl EscCode for SetStyleInArea {
    const OPCODE: u16 = 0x32;
    fn args(&self) -> Vec<String> {
        vec![self.0.encode(), self.1.encode()]
    }
}

pub struct DefaultStyleInArea(pub Area);

impl EscCode for DefaultStyleInArea {
    const OPCODE: u16 = 0x32;
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}
