pub mod args;
pub mod cmds;

pub enum MediaFormat {
    Png, Gif, Jpeg,
}

impl MediaFormat {
    fn mime(&self) -> &'static str {
        match *self {
            MediaFormat::Png    => "image/png",
            MediaFormat::Gif    => "image/gif",
            MediaFormat::Jpeg   => "image/jpeg",
        }
    }
}
