use args::{Argument, Area};
use cmds::EscCode;

pub struct Erase {
    pub area: Area,
}

impl Erase {
    pub fn new(area: Area) -> Erase {
        Erase { area: area }
    }
}

impl EscCode for Erase {
    fn opcode() -> &'static str { "20" }
    fn args(&self) -> Vec<String> {
        vec![self.area.encode()]
    }
}

pub struct RemoveChars {
    pub count: u32,
}

impl RemoveChars {
    pub fn new(count: u32) -> RemoveChars {
        RemoveChars { count: count }
    }
}

impl EscCode for RemoveChars {
    fn opcode() -> &'static str { "21" }
    fn args(&self) -> Vec<String> {
        vec![self.count.encode()]
    }
}

pub struct RemoveRows {
    pub count: u32,
    pub include: bool,
}

impl RemoveRows {
    pub fn new(count: u32, include_cu_row: bool) -> RemoveRows {
        RemoveRows {
            count: count,
            include: include_cu_row,
        }
    }
}

impl EscCode for RemoveRows {
    fn opcode() -> &'static str { "22" }
    fn args(&self) -> Vec<String> {
        vec![self.count.encode(), self.include.encode()]
    }
}

pub struct InsertBlank {
    pub count: u32
}

impl InsertBlank {
    pub fn new(count: u32) -> InsertBlank {
        InsertBlank {
            count: count,
        }
    }
}

impl EscCode for InsertBlank {
    fn opcode() -> &'static str { "26" }
    fn args(&self) -> Vec<String> {
        vec![self.count.encode()]
    }
}

pub struct InsertRows {
    pub count: u32,
    pub include: bool
}

impl InsertRows {
    pub fn new(count: u32, include_cu_row: bool) -> InsertRows {
        InsertRows {
            count: count,
            include: include_cu_row,
        }
    }
}

impl EscCode for InsertRows {
    fn opcode() -> &'static str { "27" }
    fn args(&self) -> Vec<String> {
        vec![self.count.encode(), self.include.encode()]
    }
}
