use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

mod convert;
mod display;

/// The result type of this crate.
pub type YuriResult<T> = std::result::Result<T, YuriError>;

/// A boxed error kind, wrapping an [YuriErrorKind].
#[derive(Clone)]
pub struct YuriError {
    kind: Box<YuriErrorKind>,
}

/// The kind of [YuriError].
#[derive(Debug, Clone)]
pub enum YuriErrorKind {
    GameNotFound,
    GameNotStart,
    /// An unknown error.
    UnknownError,
    SystemError {
        win32: windows::core::Error,
    },
}
