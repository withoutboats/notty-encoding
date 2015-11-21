use args::{Argument, BufferSettings, EchoSettings};
use cmds::EscCode;

pub struct SetBufferMode(pub Option<BufferSettings>);

impl EscCode for SetBufferMode {
    fn opcode() -> &'static str { "84" }
    fn args(&self) -> Vec<String> {
        if let Some(set) = self.0 {
            vec![set.encode()]
        } else { vec![] }
    }
}

pub struct SetEchoMode(pub Option<EchoSettings>);

impl EscCode for SetEchoMode {
    fn opcode() -> &'static str { "88" }
    fn args(&self) -> Vec<String> {
        if let Some(set) = self.0 {
            vec![set.encode()]
        } else { vec![] }
    }
}
