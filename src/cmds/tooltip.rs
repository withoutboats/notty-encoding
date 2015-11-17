use args::{Argument, Coords};
use cmds::EscCode;

pub struct AddToolTip(pub Coords, pub String);

impl EscCode for AddToolTip {
    fn opcode() -> &'static str { "50" }
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
    fn opcode() -> &'static str { "51" }
    fn args(&self) -> Vec<String> {
        vec![self.coords.encode()]
    }
    fn attachments(&self) -> Vec<Vec<u8>> {
        self.options.iter().cloned().map(String::into_bytes).collect()
    }
}

pub struct RemoveToolTip(pub Coords);

impl EscCode for RemoveToolTip {
    fn opcode() -> &'static str { "54" }
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}
