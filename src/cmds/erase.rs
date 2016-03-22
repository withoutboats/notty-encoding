use args::{Argument, Area};
use cmds::EscCode;

/// Command to erase all content in an area, replacing it with empty cells.
pub struct Erase {
    pub area: Area,
}

impl Erase {
    pub fn new(area: Area) -> Erase {
        Erase { area: area }
    }
}

impl EscCode for Erase {
    const OPCODE: u16 = 0x20;
    fn args(&self) -> Vec<String> {
        encode_args![self.area]
    }
}

/// Command to remove characters, beginning at the cursor. This is not the same as erasing these
/// characters: characters to the right of them will move to the left to fill their position
/// (though characters from the next line will not wrap into this line).
pub struct RemoveChars {
    pub count: u32,
}

impl RemoveChars {
    pub fn new(count: u32) -> RemoveChars {
        RemoveChars { count: count }
    }
}

impl EscCode for RemoveChars {
    const OPCODE: u16 = 0x21;
    fn args(&self) -> Vec<String> {
        encode_args![self.count]
    }
}

/// Command to remove rows, beginning at the cursor. Lines below them will move upward to replace
/// them; the boolean determines if the row the cursor is in is included in the content to be
/// removed.
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
    const OPCODE: u16 = 0x22;
    fn args(&self) -> Vec<String> {
        encode_args![self.count, self.include]
    }
}

/// Insert blank characters at the cursor, this will insert empty cells, and move the character
/// at the cursor over to the right (it will not wrap, but instead be removed from the screen).
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
    const OPCODE: u16 = 0x26;
    fn args(&self) -> Vec<String> {
        encode_args![self.count]
    }
}

/// Insert blank rows at the cursor. The boolean determines if they are inserted above the cursor
/// row, or below.
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
    const OPCODE: u16 = 0x27;
    fn args(&self) -> Vec<String> {
        encode_args![self.count, self.include]
    }
}
