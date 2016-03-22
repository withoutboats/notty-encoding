pub trait EscCode {

    const OPCODE: u16;

    fn args(&self) -> Vec<String> {
        Vec::new()
    }

    fn attachments(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    fn encode(&self) -> String {
        let mut string = format!("\x1b_[{:x}", Self::OPCODE);
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
        string + "\u{9c}"
    }

}

macro_rules! encode_args {
    (? $arg:expr, $($rest:tt)*) => {{
        encode_args!(Vec::<String>::new() => ? $arg, $($rest)*)
    }};
    (? $arg:expr) => {{
        encode_args!(Vec::<String>::new() => ? $arg)
    }};
    ($arg:expr, $($rest:tt)*) => {{
        encode_args!(Vec::<String>::new() => $arg, $($rest)*)
    }};
    ($arg:expr) => {{
        encode_args!(Vec::<String>::new() => $arg)
    }};
    ($vec:expr => ? $arg:expr, $($rest:tt)*) => {{
        $arg.map(|arg| $vec.push(arg.encode()));
        encode_args!($vec => $($rest)*)
    }};
    ($vec:expr => ? $arg:expr) => {{
        $arg.map(|arg| $vec.push(arg.encode()));
        $vec
    }};
    ($vec:expr => $arg:expr, $($rest:tt)*) => {{
        $vec.push($arg.encode());
        encode_args!($vec => $($rest)*)
    }};
    ($vec:expr => $arg:expr) => {{
        $vec.push($arg.encode());
        $vec
    }};
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
