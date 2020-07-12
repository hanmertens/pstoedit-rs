//! Rust bindings to [pstoedit](htp://pstoedit.net).
//!
//! This crate contains Rust bindings to pstoedit, a C++ program that can
//! translate PostScript and PDF graphics into other vector formats.
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

mod command;
pub mod driver_info;
mod error;

use pstoedit_sys as ffi;
use std::ffi::{CStr, CString};
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
/// See [`pstoedit`][pstoedit#examples].
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

/// Main way to interact with pstoedit.
///
/// The slice `args` represents the arguments passed to pstoedit. In the first
/// position, the name of the program (e.g. pstoedit) is expected. The parameter
/// `gs` can be used to specify the command to be used to call ghostscript. If
/// `None`, pstoedit will try to determine it automatically.
///
/// # Examples
/// ```
/// pstoedit::init()?;
/// pstoedit::pstoedit(&["pstoedit", "-gstest"], None)?;
/// # Ok::<(), pstoedit::Error>(())
/// ```
///
/// ```no_run
/// pstoedit::init()?;
/// let cmd = ["pstoedit", "-f", "latex2e", "input.ps", "output.tex"];
/// pstoedit::pstoedit(&cmd, None)?;
/// # Ok::<(), pstoedit::Error>(())
/// ```
///
/// # Errors
/// - [`NotInitialized`][Error::NotInitialized] if [`init`] was not called
/// successfully.
/// - [`NulError`][Error::NulError] if a passed string contains an internal nul
/// byte.
/// - [`PstoeditError`][Error::PstoeditError] if pstoedit returns with a
/// non-zero status code.
pub fn pstoedit<S>(args: &[S], gs: Option<S>) -> Result<()>
where
    S: AsRef<str>,
{
    let argc = args.len();
    let mut argv = SmallVec::with_capacity(argc);
    for arg in args {
        argv.push(CString::new(arg.as_ref())?);
    }
    let gs = gs.map(|s| CString::new(s.as_ref())).transpose()?;
    pstoedit_cstr(&argv, gs)
}

/// Main way to interact with pstoedit, transferring ownership of arguments.
///
/// See [`pstoedit`] for more information. The only difference is that this
/// method takes ownership of its arguments. This can be advantageous as it may
/// involve less copying and allocations, but it can also be less ergonomic. The
/// main use case is passing a [`String`] vector or iterator.
///
/// # Examples
/// ```no_run
/// pstoedit::init()?;
/// pstoedit::pstoedit_owned(std::env::args(), None)?;
/// # Ok::<(), pstoedit::Error>(())
/// ```
pub fn pstoedit_owned<I, S>(args: I, gs: Option<S>) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: Into<Vec<u8>>,
{
    let args = args.into_iter();
    let argc_min = args.size_hint().0;
    let mut argv = SmallVec::with_capacity(argc_min);
    for arg in args {
        argv.push(CString::new(arg.into())?);
    }
    let gs = gs.map(|s| CString::new(s.into())).transpose()?;
    pstoedit_cstr(&argv, gs)
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
    use std::env;

    #[test]
    fn test_init() {
        init().unwrap();
    }

    #[test]
    fn test_pstoedit() {
        init().unwrap();
        // Ensure ghostscript is not obtained through environment
        env::set_var("GS", "should_not_be_used");
        pstoedit(&["pstoedit", "-gstest"], Some("gs")).unwrap();
    }

    #[test]
    fn test_pstoedit_owned() {
        init().unwrap();
        // Ensure ghostscript is not obtained through environment
        env::set_var("GS", "should_not_be_used");
        pstoedit_owned(vec!["pstoedit", "-gstest"], Some("gs")).unwrap();
    }
}
