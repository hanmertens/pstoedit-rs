use std::ffi::NulError;
use std::os::raw::c_int;
use std::str::Utf8Error;
use std::{error, fmt, result};

/// Enumerations of possible errors during interaction with pstoedit.
#[derive(Debug)]
pub enum Error {
    /// The connection to pstoedit was not initialized, i.e.
    /// [`init`][crate::init] was not called first.
    NotInitialized,
    /// Version of pstoedit is incompatible with compiled crate.
    ///
    /// This may be because the version of pstoedit is incompatible with this
    /// crate, or the incorrect feature flags were used to specify the pstoedit
    /// version, see [the top-level documentation][crate#compatibility].
    IncompatibleVersion,
    /// Internal pstoedit (or ghostscript) error.
    PstoeditError(c_int),
    /// A UTF-8 string to be passed to pstoedit contained a nul byte.
    NulError(NulError),
    /// A string from pstoedit was invalid UTF-8.
    Utf8Error(Utf8Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::NotInitialized => None,
            Error::IncompatibleVersion => None,
            Error::PstoeditError(_) => None,
            Error::NulError(err) => Some(err),
            Error::Utf8Error(err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotInitialized => write!(f, "pstoedit was not initialized"),
            Error::IncompatibleVersion => write!(f, "incompatible pstoedit version"),
            Error::PstoeditError(err) => write!(f, "internal pstoedit error code {}", err),
            Error::NulError(err) => err.fmt(f),
            Error::Utf8Error(err) => err.fmt(f),
        }
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Self::NulError(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

/// Type of the result returned by many methods.
pub type Result<T> = result::Result<T, Error>;
