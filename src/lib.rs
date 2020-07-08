//! Rust bindings to [pstoedit](htp://pstoedit.net/).
//!
//! This crate contains Rust bindings to pstoedit, a C++ program that can
//! translate PostScript and PDF graphics into other vector formats.
//!
//! # Usage
//! First, the [`init`] function must be called. Then, other functions can be
//! called to interact with pstoedit. The main function for this is
//! [`pstoedit`]. Additionally, information about drivers can be obtained using
//! [`DriverInfo`].
//!
//! # Examples
//! ```no_run
//! pstoedit::init().unwrap();
//!
//! // For every driver ...
//! for driver in &pstoedit::DriverInfo::get().unwrap() {
//!     let format = driver.symbolic_name().unwrap();
//!     let extension = driver.extension().unwrap();
//!     let output_name = format!("output-{}.{}", format, extension);
//!
//!     // ... convert input.ps to that format
//!     let cmd = ["pstoedit", "-f", format, "input.ps", output_name.as_ref()];
//!     pstoedit::pstoedit(&cmd, None).unwrap();
//! }
//! ```

pub mod driver_info;
mod error;

use pstoedit_sys as ffi;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

pub use driver_info::DriverInfo;
pub use error::{Error, Result};

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
/// The slice `args` represents the arguments passed to pstoedit. The argument
/// `gs` can be used to specify the command to be used to call ghostscript. If
/// `None`, pstoedit will try to determine it automatically.
///
/// # Examples
/// ```
/// pstoedit::init().unwrap();
/// pstoedit::pstoedit(&["pstoedit", "-gstest"], None).unwrap();
/// ```
///
/// ```no_run
/// pstoedit::init().unwrap();
/// let cmd = ["pstoedit", "-f", "latex2e", "input.ps", "output.tex"];
/// pstoedit::pstoedit(&cmd, None).unwrap();
/// ```
///
/// # Errors
/// - [`NotInitialized`][Error::NotInitialized] if [`init`] was not called
/// succesfully.
/// - [`NulError`][Error::NulError] if a passed string contains an internal nul
/// byte.
/// - [`PstoeditError`][Error::PstoeditError] if pstoedit returns with a
/// non-zero status code.
pub fn pstoedit<S>(args: &[S], gs: Option<S>) -> Result<()>
where
    S: AsRef<str>,
{
    let argc = args.len();
    let mut argv = Vec::with_capacity(argc);
    for arg in args {
        argv.push(CString::new(arg.as_ref())?);
    }
    let gs = gs.map(|s| CString::new(s.as_ref())).transpose()?;
    pstoedit_cstr(&argv, gs)
}

/// Thin safe wrapper to main pstoedit API.
///
/// Safety is ensured using the invariants of [`CStr`].
fn pstoedit_cstr<S>(argv: &[S], gs: Option<S>) -> Result<()>
where
    S: AsRef<CStr>,
{
    let argv: Vec<_> = argv.iter().map(|s| s.as_ref().as_ptr()).collect();
    // First as_ref is required to prevent CString from being moved and dropped
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
}
