//! Rust bindings to [pstoedit](htp://pstoedit.net).
//!
//! This crate contains Rust bindings to pstoedit, a C++ program that can
//! translate PostScript and PDF graphics into other vector formats.
//!
//! # Compatiblity
//! Multiple versions of pstoedit are supported, but the appropriate feature
//! starting with `pstoedit_` has to be enabled.
//!
//! - `pstoedit_4_00`: compatible with pstoedit version 4.00&ndash;4.01, and likely
//!   with future 4.xx releases.
//! - No feature starting with `pstoedit_`: compatible with pstoedit version
//!   3.17&ndash;3.78.
//!
//! # Usage
//! First, the [`init`] function must be called. Then, interaction with pstoedit
//! is possible using [`Command`] or [`DriverInfo`].
//!
//! # Examples
//! ```no_run
//! use pstoedit::{DriverInfo, Command};
//!
//! pstoedit::init()?;
//!
//! // For every driver ...
//! for driver in &DriverInfo::get()? {
//!     let format = driver.symbolic_name()?;
//!     let extension = driver.extension()?;
//!     let output_name = format!("output-{}.{}", format, extension);
//!
//!     // ... convert input.ps to that format
//!     Command::new().args_slice(&["-f", format, "input.ps"])?.arg(output_name)?.run()?;
//! }
//! # Ok::<(), pstoedit::Error>(())
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

mod command;
pub mod driver_info;
mod error;

use pstoedit_sys as ffi;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::ptr;

pub use command::Command;
pub use driver_info::DriverInfo;
pub use error::{Error, Result};

#[cfg(feature = "smallvec")]
type SmallVec<T> = smallvec::SmallVec<[T; 5]>;
#[cfg(feature = "smallvec")]
use smallvec::smallvec;
#[cfg(not(feature = "smallvec"))]
type SmallVec<T> = Vec<T>;
#[cfg(not(feature = "smallvec"))]
use vec as smallvec;

/// Initialize connection to pstoedit. Must be called before calling any other
/// function that requires a connection to pstoedit.
///
/// # Examples
/// See [`Command`][Command#examples].
///
/// # Errors
/// [`IncompatibleVersion`][Error::IncompatibleVersion] if the version of
/// pstoedit is not compatible with this crate.
pub fn init() -> Result<()> {
    if unsafe { ffi::pstoedit_checkversion(ffi::pstoeditdllversion) } != 0 {
        Ok(())
    } else {
        Err(Error::IncompatibleVersion)
    }
}

/// Thin safe wrapper to main pstoedit API.
///
/// Safety is ensured using the invariants of [`CStr`].
fn pstoedit_cstr<S, T>(argv: &[S], gs: Option<T>) -> Result<()>
where
    S: AsRef<CStr>,
    T: AsRef<CStr>,
{
    let argv: SmallVec<_> = argv.iter().map(|s| s.as_ref().as_ptr()).collect();
    // First as_ref is required to prevent move and drop if T = CString
    let gs = gs.as_ref().map_or(ptr::null(), |s| s.as_ref().as_ptr());
    // Safety: due to CStr input arguments it is ensured they are valid C strings
    unsafe { pstoedit_raw(&argv, gs) }
}

/// Thin wrapper to main pstoedit API that sets `argc` and converts errors.
///
/// # Safety
/// All pointers must be valid C strings; `gs` may be null.
unsafe fn pstoedit_raw(argv: &[*const c_char], gs: *const c_char) -> Result<()> {
    debug_assert!(argv.len() <= c_int::MAX as usize);
    let argc = argv.len() as c_int;
    pstoedit_result(ffi::pstoedit_plainC(argc, argv.as_ptr(), gs))
}

/// Interpret pstoedit return value as result.
fn pstoedit_result(error_code: c_int) -> Result<()> {
    match error_code {
        0 => Ok(()),
        -1 => Err(Error::NotInitialized),
        err => Err(Error::PstoeditError(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init().unwrap();
    }
}
