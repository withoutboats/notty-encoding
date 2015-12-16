pub trait EscCode {

    fn opcode() -> &'static str;

    fn args(&self) -> Vec<String> {
        Vec::new()
    }

    fn attachments(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    fn encode(&self) -> String {
        let mut string = String::from("\x1b_[");
        string.push_str(Self::opcode());
        for arg in self.args() {
            string.push(';');
            string.push_str(&arg);
        }
        for attachment in self.attachments() {
            string.push('#');
            string.push_str(unsafe {
                &String::from_utf8_unchecked(::base64::u8en(&attachment).unwrap())
            });
        }
        string.push_str("\u{9c}");
        string
    }

}

mod erase;
mod meta;
mod movement;
mod put;
mod style;
mod tooltip;

pub use self::erase::{Erase, RemoveChars, RemoveRows, InsertBlank, InsertRows};
pub use self::meta::{SetTitle, PushBuffer, PopBuffer, SetInputMode, HoldForInput};
pub use self::movement::{Move, ScrollScreen};
pub use self::put::{PutMedia, PutMediaAt};
pub use self::style::{SetTextStyle, DefaultTextStyle, SetCursorStyle, DefaultCursorStyle,
                      SetStyleInArea, DefaultStyleInArea};
pub use self::tooltip::{AddToolTip, AddDropDown, RemoveToolTip};
