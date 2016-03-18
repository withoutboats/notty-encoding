use args::{Argument, Coords, MediaPosition};
use cmds::EscCode;
use MediaFormat;

pub struct PutMedia {
    width: u32,
    height: u32,
    position: MediaPosition,
    fmt: MediaFormat,
    data: Vec<u8>
}

impl PutMedia {
    pub fn new(width: u32,
               height: u32,
               position: MediaPosition,
               fmt: MediaFormat,
               data: Vec<u8>) -> PutMedia {
        PutMedia {
            width: width, height: height, position: position, fmt: fmt, data: data
        }
    }
}

impl EscCode for PutMedia {
    const OPCODE: u16 = 0x14;
    fn args(&self) -> Vec<String> {
        vec![self.width.encode(), self.height.encode(), self.position.encode()]
    }
    fn attachments(&self) -> Vec<Vec<u8>> {
        vec![Vec::from(self.fmt.mime().as_bytes()), self.data.clone()]
    }
}

pub struct PutMediaAt {
    coords: Coords,
    width: u32,
    height: u32,
    position: MediaPosition,
    fmt: MediaFormat,
    data: Vec<u8>
}

impl PutMediaAt {
    pub fn new(coords: Coords,
               width: u32,
               height: u32,
               position: MediaPosition,
               fmt: MediaFormat,
               data: Vec<u8>) -> PutMediaAt {
        PutMediaAt {
            coords: coords, width: width, height: height, position: position, fmt: fmt, data: data
        }
    }
}
impl EscCode for PutMediaAt {
    const OPCODE: u16 = 0x15;
    fn args(&self) -> Vec<String> {
        vec![self.coords.encode(), self.width.encode(), self.height.encode(),
             self.position.encode()]
    }
    fn attachments(&self) -> Vec<Vec<u8>> {
        vec![Vec::from(self.fmt.mime().as_bytes()), self.data.clone()]
    }
}
