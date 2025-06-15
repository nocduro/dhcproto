//! Error types for Encoding/Decoding
use std::{error, fmt, io};

/// Convenience type for decode errors
pub type DecodeResult<T> = Result<T, DecodeError>;

/// Returned from types that decode
#[derive(Debug)]
pub enum DecodeError {
    /// add overflow
    AddOverflow,

    /// ran out of bytes
    NotEnoughBytes,

    /// error converting from slice
    SliceError(std::array::TryFromSliceError),

    /// error finding nul in string
    NulError(std::ffi::FromBytesWithNulError),

    /// error converting to utf-8
    Utf8Error(std::str::Utf8Error),

    /// io error
    IoError(io::Error),

    /// Unknown decode error
    Unknown(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<std::array::TryFromSliceError> for DecodeError {
    fn from(value: std::array::TryFromSliceError) -> Self {
        DecodeError::SliceError(value)
    }
}

impl From<std::ffi::FromBytesWithNulError> for DecodeError {
    fn from(value: std::ffi::FromBytesWithNulError) -> Self {
        DecodeError::NulError(value)
    }
}

impl From<std::str::Utf8Error> for DecodeError {
    fn from(value: std::str::Utf8Error) -> Self {
        DecodeError::Utf8Error(value)
    }
}

impl From<io::Error> for DecodeError {
    fn from(value: io::Error) -> Self {
        DecodeError::IoError(value)
    }
}

impl error::Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::AddOverflow => f.write_str("decoder checked_add failed"),
            DecodeError::NotEnoughBytes => f.write_str("parser ran out of data-- not enough bytes"),
            DecodeError::SliceError(try_from_slice_error) => {
                write!(f, "error converting from slice {0}", try_from_slice_error)
            }
            DecodeError::NulError(n) => write!(f, "error getting null terminated string {0}", n),
            DecodeError::Utf8Error(utf8_error) => {
                write!(f, "error converting to UTF-8 {0}", utf8_error)
            }
            DecodeError::IoError(error) => write!(f, "io error {0}", error),
            DecodeError::Unknown(error) => write!(f, "unknown error {0}", error),
        }
    }
}

/// Returned from types that encode
#[derive(Debug)]
pub enum EncodeError {
    /// addition overflow
    AddOverflow,

    /// string exceeds bounds
    StringSizeTooBig {
        /// size of string
        len: usize,
    },

    /// io error
    IoError(io::Error),
}

impl error::Error for EncodeError {}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodeError::AddOverflow => write!(f, "encoder checked_add failed"),
            EncodeError::StringSizeTooBig { len } => write!(f, "message is trying to write a string to the message that exceeds the max size of {len}"),
            EncodeError::IoError(error) => write!(f, "io error {error}"),
        }
    }
}

impl From<io::Error> for EncodeError {
    fn from(value: io::Error) -> Self {
        EncodeError::IoError(value)
    }
}

/// Convenience type for encode errors
pub type EncodeResult<T> = Result<T, EncodeError>;
