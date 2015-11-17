use args::{Argument, InputMode};
use cmds::EscCode;

pub struct SetTitle(pub String);

impl EscCode for SetTitle {
    fn opcode() -> &'static str { "40" }
    fn attachments(&self) -> Vec<Vec<u8>> {
        vec![self.0.clone().into_bytes()]
    }
}

pub struct PushBuffer(pub bool);

impl EscCode for PushBuffer {
    fn opcode() -> &'static str { "60" }
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}

pub struct PopBuffer;

impl EscCode for PopBuffer {
    fn opcode() -> &'static str { "61" }
}

pub struct SetInputMode(pub InputMode);

impl EscCode for SetInputMode {
    fn opcode() -> &'static str { "80" }
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}
