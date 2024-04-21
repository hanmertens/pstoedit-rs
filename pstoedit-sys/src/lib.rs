//! Native bindings to [pstoedit](http://pstoedit.net).
//!
//! This crate contains low-level bindings to the C API of pstoedit, a C++
//! program that can translate PostScript and PDF graphics into other vector
//! formats.
//!
//! # Compatibility
//! Multiple versions of pstoedit are supported, but the appropriate feature
//! starting with `pstoedit_` has to be enabled.
//!
//! - `pstoedit_4_01`: compatible with pstoedit version 4.01, and likely with future
//!   4.xx releases.
//! - `pstoedit_4_00`: compatible with pstoedit version 4.00&ndash;4.01, and likely
//!   with future 4.xx releases.
//! - No feature starting with `pstoedit_`: compatible with pstoedit version
//!   3.17&ndash;3.78.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod bindings;

pub use bindings::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::{c_char, c_int};
    use std::{env, ptr};

    #[test]
    fn dll_version() {
        #[cfg(not(feature = "pstoedit_4_00"))]
        assert_eq!(pstoeditdllversion, 301);
        #[cfg(feature = "pstoedit_4_00")]
        assert_eq!(pstoeditdllversion, 401);
    }

    #[test]
    fn init() {
        assert!(unsafe { pstoedit_checkversion(pstoeditdllversion) } != 0);
    }

    #[test]
    fn driver_info() {
        init();
        let drivers: *mut DriverDescription_S = unsafe { getPstoeditDriverInfo_plainC() };
        assert!(drivers != ptr::null_mut());
        unsafe { clearPstoeditDriverInfo_plainC(drivers) };
    }

    #[test]
    fn native_driver_info() {
        init();
        let drivers: *mut DriverDescription_S = unsafe { getPstoeditNativeDriverInfo_plainC() };
        assert!(drivers != ptr::null_mut());
        unsafe { clearPstoeditDriverInfo_plainC(drivers) };
    }

    #[test]
    fn pstoedit() {
        init();
        // Perform ghostscript test
        let argv = [
            b"pstoedit\0".as_ptr() as *const c_char,
            b"-gstest\0".as_ptr() as *const c_char,
        ];
        let argc = argv.len() as c_int;
        // Get ghostscript through string, not environment
        let psinterpreter = b"gs\0".as_ptr() as *const c_char;
        env::set_var("GS", "should_not_be_used");
        let result = unsafe { pstoedit_plainC(argc, argv.as_ptr(), psinterpreter) };
        assert_eq!(result, 0);
    }
}
