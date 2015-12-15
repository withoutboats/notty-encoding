pub trait EscCode {

    fn opcode() -> &'static str;

    fn args(&self) -> Vec<String> {
        Vec::new()
    }

    fn attachments(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    fn encode(&self) -> Vec<u8> {
        let mut vec = vec![0x1b, b'{'];
        vec.extend(Self::opcode().as_bytes());
        for arg in self.args() {
            vec.push(b';');
            vec.extend(arg.as_bytes());
        }
        for attachment in self.attachments() {
            vec.push(b'{');
            vec.extend(format!("{:x}", attachment.len()).as_bytes());
            vec.push(b';');
            vec.extend(attachment);
        }
        vec.push(b'}');
        vec
    }

}

mod erase;
mod meta;
mod movement;
mod put;
mod style;
mod tooltip;

pub use self::erase::{Erase, RemoveChars, RemoveRows, InsertBlank, InsertRows};
pub use self::meta::{SetTitle, PushBuffer, PopBuffer, SetInputMode, HoldForBuffer};
pub use self::movement::{Move, ScrollScreen};
pub use self::put::{PutMedia, PutMediaAt};
pub use self::style::{SetTextStyle, DefaultTextStyle, SetCursorStyle, DefaultCursorStyle,
                      SetStyleInArea, DefaultStyleInArea};
pub use self::tooltip::{AddToolTip, AddDropDown, RemoveToolTip};
