use args::{Argument, InputSettings};
use cmds::EscCode;

/// Set the title of the screen.
pub struct SetTitle(pub String);

impl EscCode for SetTitle {
    const OPCODE: u16 = 0x40;
    fn attachments(&self) -> Vec<Vec<u8>> {
        vec![self.0.clone().into_bytes()]
    }
}

/// Set the input mode for the terminal.
pub struct SetInputMode(pub InputSettings);

impl EscCode for SetInputMode {
    const OPCODE: u16 = 0x80;
    fn args(&self) -> Vec<String> {
        encode_args![self.0]
    }
}

/// In local echo mode, do not process non-input commands until input has been sent.
pub struct HoldForInput;

impl EscCode for HoldForInput {
    const OPCODE: u16 = 0x87;
}
