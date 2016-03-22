use args::{Argument, Coords};
use cmds::EscCode;

/// Add a tooltip at a given coordinate.
pub struct AddToolTip(pub Coords, pub String);

impl EscCode for AddToolTip {
    const OPCODE: u16 = 0x50;
    fn args(&self) -> Vec<String> {
        encode_args![self.0]
    }
    fn attachments(&self) -> Vec<Vec<u8>> {
        vec![self.1.clone().into_bytes()]
    }
}

/// Add a dropdown menu at a given coordinate.
pub struct AddDropDown {
    coords: Coords,
    options: Vec<String>,
}

impl EscCode for AddDropDown {
    const OPCODE: u16 = 0x51;
    fn args(&self) -> Vec<String> {
        encode_args![self.coords]
    }
    fn attachments(&self) -> Vec<Vec<u8>> {
        self.options.iter().cloned().map(String::into_bytes).collect()
    }
}

/// Remove a tooltip or dropdown from a given coordinate, if there is one there.
pub struct RemoveToolTip(pub Coords);

impl EscCode for RemoveToolTip {
    const OPCODE: u16 = 0x54;
    fn args(&self) -> Vec<String> {
        encode_args![self.0]
    }
}
