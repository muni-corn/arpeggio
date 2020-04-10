use std::fmt;
use std::num::{ParseIntError, TryFromIntError};

#[derive(Debug)]
pub enum ArpeggioError {
    Basic(BasicError),
    Io(std::io::Error),
    Image(image::error::ImageError),
    ParseInt(ParseIntError),
    TryFromInt(TryFromIntError),
    Serde(String),
}

impl From<BasicError> for ArpeggioError {
    fn from(e: BasicError) -> Self {
        Self::Basic(e)
    }
}

impl From<std::io::Error> for ArpeggioError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<image::error::ImageError> for ArpeggioError {
    fn from(e: image::error::ImageError) -> Self {
        Self::Image(e)
    }
}

impl From<ParseIntError> for ArpeggioError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<TryFromIntError> for ArpeggioError {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromInt(e)
    }
}

impl fmt::Display for ArpeggioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic(b) => b.fmt(f),
            Self::Io(i) => i.fmt(f),
            Self::Image(i) => i.fmt(f),
            Self::ParseInt(p) => p.fmt(f),
            Self::TryFromInt(t) => t.fmt(f),
            Self::Serde(s) => s.fmt(f),
        }
    }
}

impl std::error::Error for ArpeggioError {}

impl serde::de::Error for ArpeggioError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Serde(msg.to_string())
    }
}

#[derive(Debug)]
pub struct BasicError {
    pub message: String,
}

impl fmt::Display for BasicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "arpeggio came across a problem: {}", self.message)
    }
}
