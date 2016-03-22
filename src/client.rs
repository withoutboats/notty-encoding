use std::io::{self, Write};

use cmds::EscCode;

pub trait Client {
    fn write(&mut self, &EscCode) -> io::Result<()>;
}

pub struct StdioClient;

impl Client for StdioClient {
    fn write(&mut self, code: &EscCode) -> io::Result<()> {
        io::stdout().write_all(code.encode().as_bytes())
    }
}
