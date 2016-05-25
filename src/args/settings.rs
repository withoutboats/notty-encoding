use super::Argument;
use self::InputSettings::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferSettings {
    pub eol1: u8,
    pub eol2: u8,
    pub eof: u8,
    pub intr: u8,
    pub quit: u8,
    pub susp: u8,
}

impl BufferSettings {
    pub fn eof(&self, c: char) -> bool {
        if let '\0'...'\x7f' = c {
            c as u8 == self.eof
        } else { false }
    }

    pub fn eol(&self, c: char) -> bool {
        if let '\0'...'\x7f' = c {
            c as u8 == self.eol1 || c as u8 == self.eol2 || c as u8 == self.eof
        } else { false }
    }

    pub fn signal(&self, c: char) -> bool {
        if let '\0'...'\x7f' = c {
            c as u8 == self.intr || c as u8 == self.quit || c as u8 == self.susp
        } else { false }
    }
}

impl Argument for BufferSettings {
    fn from_nums<T>(mut args: T, default: Option<BufferSettings>) -> Option<BufferSettings>
    where T: Iterator<Item=u64> {
        let intr = match args.next() { Some(n) => n as u8, None => return default };
        let quit = match args.next() { Some(n) => n as u8, None => return default };
        let susp = match args.next() { Some(n) => n as u8, None => return default };
        let eol1 = match args.next() { Some(n) => n as u8, None => return default };
        let eol2 = match args.next() { Some(n) => n as u8, None => return default };
        let eof  = match args.next() { Some(n) => n as u8, None => return default };
        Some(BufferSettings {
            eol1: eol1,
            eol2: eol2,
            eof: eof,
            intr: intr,
            quit: quit,
            susp: susp,
        })
    }

    fn encode(&self) -> String {
        format!("{:x}.{:x}.{:x}.{:x}.{:x}.{:x}", self.intr, self.quit, self.susp, self.eol1,
                self.eol2, self.eof)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EchoSettings {
    pub lerase: u8,
    pub lnext: u8,
    pub werase: u8,
}

impl Argument for EchoSettings {
    fn from_nums<T>(mut args: T, default: Option<EchoSettings>) -> Option<EchoSettings>
    where T: Iterator<Item=u64> {
        let lerase = match args.next() { Some(n) => n as u8, None => return default };
        let lnext = match args.next() { Some(n) => n as u8, None => return default };
        let werase = match args.next() { Some(n) => n as u8, None => return default };
        Some(EchoSettings {
            lerase: lerase,
            lnext: lnext,
            werase: werase,
        })
    }

    fn encode(&self) -> String {
        format!("{:x}.{:x}.{:x}", self.lerase, self.lnext, self.werase)
    }
}

/// The mode the input processor is in.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InputSettings {
    /// Ansi-compatible mode, boolean determines of "application" mode or not.
    Ansi(bool),
    /// Bracketed-paste mode (on or off)
    BracketedPasteMode(bool),
    /// Notty mode.
    Notty(()),
    LineBufferEcho(EchoSettings, BufferSettings),
    ScreenEcho(EchoSettings),
}

impl Argument for InputSettings {
    fn from_nums<T>(mut args: T, default: Option<InputSettings>) -> Option<InputSettings>
    where T: Iterator<Item=u64> {
        match args.next() {
            Some(1) => Some(Ansi(false)),
            Some(2) => Some(Notty(())),
            Some(3) => EchoSettings::from_nums(args.by_ref(), None).and_then(|echo| {
                BufferSettings::from_nums(args, None).map(|buffer| LineBufferEcho(echo, buffer))
            }),
            Some(4) => EchoSettings::from_nums(args, None).map(ScreenEcho),
            _       => default,
        }
    }

    fn encode(&self) -> String {
        match *self {
            Ansi(_)                         => String::from("1"),
            BracketedPasteMode(_)           => unimplemented!(),
            Notty(_)                        => String::from("2"),
            LineBufferEcho(echo, buffer)    => format!("3.{}.{}", echo.encode(), buffer.encode()),
            ScreenEcho(echo)                => format!("4.{}", echo.encode()),
        }
    }
}
