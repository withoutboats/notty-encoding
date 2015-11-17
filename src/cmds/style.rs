use args::{Argument, Area, Style};
use cmds::EscCode;

pub struct SetTextStyle(pub Style);

impl EscCode for SetTextStyle {
    fn opcode() -> &'static str { "30" }
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}

pub struct DefaultTextStyle;

impl EscCode for DefaultTextStyle {
    fn opcode() -> &'static str { "30" }
}

pub struct SetCursorStyle(pub Style);

impl EscCode for SetCursorStyle {
    fn opcode() -> &'static str { "31" }
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}

pub struct DefaultCursorStyle;

impl EscCode for DefaultCursorStyle {
    fn opcode() -> &'static str { "31" }
}

pub struct SetStyleInArea(pub Area, pub Style);

impl EscCode for SetStyleInArea {
    fn opcode() -> &'static str { "32" }
    fn args(&self) -> Vec<String> {
        vec![self.0.encode(), self.1.encode()]
    }
}

pub struct DefaultStyleInArea(pub Area);

impl EscCode for DefaultStyleInArea {
    fn opcode() -> &'static str { "32" }
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}
