use args::{Argument, Coords};
use cmds::EscCode;

pub struct AddToolTip(pub Coords, pub String);

impl EscCode for AddToolTip {
    const OPCODE: u16 = 0x50;
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
    fn attachments(&self) -> Vec<Vec<u8>> {
        vec![self.1.clone().into_bytes()]
    }
}

pub struct AddDropDown {
    coords: Coords,
    options: Vec<String>,
}

impl EscCode for AddDropDown {
    const OPCODE: u16 = 0x51;
    fn args(&self) -> Vec<String> {
        vec![self.coords.encode()]
    }
    fn attachments(&self) -> Vec<Vec<u8>> {
        self.options.iter().cloned().map(String::into_bytes).collect()
    }
}

pub struct RemoveToolTip(pub Coords);

impl EscCode for RemoveToolTip {
    const OPCODE: u16 = 0x54;
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}
