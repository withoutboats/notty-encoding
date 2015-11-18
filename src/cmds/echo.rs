use args::{Argument, BufferSet};
use cmds::EscCode;

pub struct SetBufferMode(pub Option<BufferSet>);

impl EscCode for SetBufferMode {
    fn opcode() -> &'static str { "84" }
    fn args(&self) -> Vec<String> {
        if let Some(set) = self.0 {
            vec![set.encode()]
        } else { vec![] }
    }
}

pub struct SetEchoMode(pub bool);

impl EscCode for SetEchoMode {
    fn opcode() -> &'static str { "88" }
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}
