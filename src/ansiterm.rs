use core::fmt;

pub(crate) const ANSI_ESCAPE: u8 = 27;
pub(crate) const CLEAR_FORMAT: AnsiEscapeSequence =
    AnsiEscapeSequence::RenditionSelectGraphic(RenditionSelectGraphic::Reset);

#[allow(dead_code)]
pub(crate) enum RenditionSelectGraphic {
    Reset, 
    Bold,
    ForegroundColor(Color),
    BackgroundColor(Color),
    DefaultColorForeground,
    DefaultColorBackground,
}

pub(crate) enum AnsiEscapeSequence {
    RenditionSelectGraphic(RenditionSelectGraphic),
    MoveCursorTopLeft,
    ClearScreenFromCursorToEnd,
    ClearEntireLine,
}

impl fmt::Display for RenditionSelectGraphic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reset => write!(f, "0")?,
            Self::Bold => write!(f, "1")?,
            Self::ForegroundColor(color) => write!(f, "{}", color.foreground_byte())?,
            Self::BackgroundColor(color) => write!(f, "{}", color.background_byte())?,
            Self::DefaultForegroundColor => write!(f, "39")?,
            Self::DefaultBackgroundColor => write!(f, "49")?,
        }
        write!(f, "m")
    }
}

impl fmt::Display for AnsiEscapeSequence {

}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    fn foreground_byte(self) -> u8 {
        match self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
        }
    }

    fn background_byte(self) -> u8 {
        match self {
            Self::Black => 40,
            Self::Red => 41,
            Self::Green => 42,
            Self::Yellow => 43,
            Self::Blue => 44,
            Self::Magenta => 45,
            Self::Cyan => 46,
            Self::White => 47,
        }
    }
}
