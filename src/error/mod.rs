//! Custom error and result types for `algocol`. These are the types that this
//! crate uses over the `std` error and result types. The types defined here
//! are prefixed with "Agc" in front to differentiate itself from the types
//! defined in `std`.

use std::{
    convert::{Into},
    error,
    fmt
};

/// This enum is a list of possible kinds of errors that `algocol` may
/// experience. This is similar to `std::io::ErrorKind` and is used by
/// `AgcError`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgcErrorKind {
    OutOfBounds,
    WrongOrder,
    Unordered,
    AlreadyExists,
    SameNode,
    NotFound,
    Other
}

impl fmt::Display for AgcErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<String> for AgcErrorKind {
    fn into(self) -> String {
        format!("{:?}", self)
    }
}

impl<'a> Into<String> for &'a AgcErrorKind {
    fn into(self) -> String {
        format!("{:?}", self)
    }
}

/// The Error type used for this crate. This error type has a `kind` which
/// determines what class of error has occurred and a description telling you
/// what happened.
#[derive(Debug, PartialEq, Eq)]
pub struct AgcError {
    kind: AgcErrorKind,
    description: String
}

impl AgcError {
    /// Create a new `AgcError` with the class of error and a short description
    /// of what happened.
    pub fn new(kind: AgcErrorKind, description: impl AsRef<str>) -> Self {
        let description = description.as_ref().to_string();
        Self {kind, description}
    }
}

impl fmt::Display for AgcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.description)
    }
}

impl error::Error for AgcError {}

/// Special `Result` type used by `algocol`. This enum is used when you expect
/// that an error may occur and want to return the error to the user when
/// it happens.
pub type AgcResult<T> = std::result::Result<T, AgcError>;
