//! Information on pstoedit drivers.
//!
//! Inquire information on drivers using [`DriverInfo::get`] or
//! [`DriverInfo::get_native`], and iterate over it to yield a
//! [`DriverDescription`] for each driver.
//!
//! # Examples
//! ```
//! use std::collections::HashSet;
//! use pstoedit::DriverInfo;
//!
//! pstoedit::init()?;
//!
//! let drivers = DriverInfo::get()?;
//! let native_drivers = DriverInfo::get_native()?;
//!
//! // The number of non-native drivers cannot be negative
//! let num = drivers.iter().count();
//! let num_native = native_drivers.iter().count();
//! assert!(num >= num_native);
//!
//! // Ensure all drivers have a unique symbolic name
//! let mut formats = HashSet::new();
//! for driver in &drivers {
//!     assert!(formats.insert(driver.symbolic_name()?));
//! }
//!
//! // Ensure all native drivers are included in the list of all drivers
//! for driver in &native_drivers {
//!     assert!(formats.contains(driver.symbolic_name()?));
//! }
//! # Ok::<(), pstoedit::Error>(())
//! ```

use crate::ffi;
use crate::{Error, Result};
use std::ffi::CStr;
use std::ptr::NonNull;

/// Format group of pstoedit driver.
///
/// Driver-specific options of pstoedit are specific to a format group. All
/// drivers in a format group have an equal value of `FormatGroup`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg(feature = "pstoedit_4_00")]
pub struct FormatGroup(std::ffi::c_int);

/// Description of pstoedit driver.
///
/// Information on pstoedit drivers can be obtained through [`DriverInfo`].
///
/// # Errors
/// For all methods returning `Result<&str>`, a [`Utf8Error`][Error::Utf8Error]
/// can occur when pstoedit yields invalid UTF-8, but this should only be
/// possible with non-standard drivers.
#[derive(Copy, Clone, Debug)]
pub struct DriverDescription<'a>(&'a ffi::DriverDescription_S);

impl<'a> DriverDescription<'a> {
    /// File name extension associated with the driver.
    pub fn extension(self) -> Result<&'a str> {
        Ok(unsafe { CStr::from_ptr(self.0.suffix) }.to_str()?)
    }

    /// Symbolic name to uniquely identify the driver.
    pub fn symbolic_name(self) -> Result<&'a str> {
        Ok(unsafe { CStr::from_ptr(self.0.symbolicname) }.to_str()?)
    }

    /// Short explanation of the driver.
    pub fn explanation(self) -> Result<&'a str> {
        Ok(unsafe { CStr::from_ptr(self.0.explanation) }.to_str()?)
    }

    /// Additional information about the driver.
    ///
    /// This can be, and often is, an empty string.
    pub fn additional_info(self) -> Result<&'a str> {
        Ok(unsafe { CStr::from_ptr(self.0.additionalInfo) }.to_str()?)
    }

    /// Whether the backend supports subpaths.
    pub fn subpath_support(self) -> bool {
        self.0.backendSupportsSubPaths != 0
    }

    /// Whether the backend supports curveto.
    pub fn curveto_support(self) -> bool {
        self.0.backendSupportsCurveto != 0
    }

    /// Whether the backend supports merging.
    pub fn merging_support(self) -> bool {
        self.0.backendSupportsMerging != 0
    }

    /// Whether the backend supports text.
    pub fn text_support(self) -> bool {
        self.0.backendSupportsText != 0
    }

    /// Whether the backend supports images.
    pub fn image_support(self) -> bool {
        self.0.backendSupportsImages != 0
    }

    /// Whether the backend supports multiple pages.
    pub fn multipage_support(self) -> bool {
        self.0.backendSupportsMultiplePages != 0
    }

    /// Format group of driver.
    #[cfg(feature = "pstoedit_4_00")]
    pub fn format_group(self) -> FormatGroup {
        FormatGroup(self.0.formatGroup)
    }
}

/// Information on pstoedit drivers.
///
/// See [module-level documentation][self] for more details.
// Holds pointer to first element of DriverDescription_S array
// The end of the array is indicated by an element with a null pointer as symbolicname
pub struct DriverInfo(NonNull<ffi::DriverDescription_S>);

impl DriverInfo {
    /// Inquire driver information.
    ///
    /// # Examples
    /// ```
    /// pstoedit::init().unwrap();
    /// let drivers = pstoedit::DriverInfo::get().unwrap();
    /// let names = drivers
    ///     .iter()
    ///     .map(|driver| driver.symbolic_name().unwrap())
    ///     .collect::<Vec<_>>();
    /// ```
    ///
    /// # Errors
    /// [`NotInitialized`][Error::NotInitialized] if [`init`][crate::init] was
    /// not called successfully.
    pub fn get() -> Result<Self> {
        let info = unsafe { ffi::getPstoeditDriverInfo_plainC() };
        NonNull::new(info).map(Self).ok_or(Error::NotInitialized)
    }

    /// Inquire native driver information.
    ///
    /// See [`get`][DriverInfo::get] for usage.
    pub fn get_native() -> Result<Self> {
        let info = unsafe { ffi::getPstoeditNativeDriverInfo_plainC() };
        NonNull::new(info).map(Self).ok_or(Error::NotInitialized)
    }

    /// Generate iterator over drivers in driver information.
    ///
    /// # Examples
    /// See [`get`][DriverInfo::get].
    pub fn iter(&self) -> Iter {
        Iter {
            driver_info: self,
            offset: 0,
        }
    }
}

impl Drop for DriverInfo {
    fn drop(&mut self) {
        // Hand back ownership to pstoedit for deallocation
        unsafe { ffi::clearPstoeditDriverInfo_plainC(self.0.as_ptr()) };
    }
}

impl<'a> IntoIterator for &'a DriverInfo {
    type Item = DriverDescription<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over drivers in [`DriverInfo`], yielding a [`DriverDescription`]
/// for each one.
pub struct Iter<'a> {
    driver_info: &'a DriverInfo,
    offset: isize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = DriverDescription<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Safety
        // - Our pointer is well-aligned and non-null, which still holds if an
        // offset is added
        // - Offset is no more than the array length as it is not increased when
        // it is at the final element
        unsafe {
            // Get the offset-th element
            let driver = self
                .driver_info
                .0
                .as_ptr()
                .offset(self.offset)
                .as_ref()
                .unwrap();
            // symbolicname being a null pointer indicates an exhausted list
            driver.symbolicname.as_ref().map(|_| {
                self.offset += 1;
                DriverDescription(driver)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn driver_info() {
        crate::init().unwrap();
        let drivers = DriverInfo::get().unwrap();
        assert!(!drivers.iter().next().is_none());
    }

    #[test]
    fn driver_info_native() {
        crate::init().unwrap();
        let drivers = DriverInfo::get_native().unwrap();
        assert!(!drivers.iter().next().is_none());
    }

    #[test]
    fn driver_iter() {
        crate::init().unwrap();
        for driver in &DriverInfo::get().unwrap() {
            assert!(driver.extension().is_ok());
            assert!(driver.symbolic_name().is_ok());
            assert!(driver.explanation().is_ok());
            assert!(driver.additional_info().is_ok());
        }
    }

    #[test]
    fn driver_iter_native() {
        crate::init().unwrap();
        for driver in &DriverInfo::get_native().unwrap() {
            assert!(driver.extension().is_ok());
            assert!(driver.symbolic_name().is_ok());
            assert!(driver.explanation().is_ok());
            assert!(driver.additional_info().is_ok());
        }
    }

    #[test]
    fn psf_driver() {
        crate::init().unwrap();
        let info = DriverInfo::get().unwrap();
        let driver = info
            .iter()
            .find(|x| x.symbolic_name().unwrap() == "psf")
            .unwrap();
        assert_eq!(driver.extension().unwrap(), "fps");
        assert!(driver.subpath_support());
        assert!(!driver.curveto_support());
        assert!(driver.merging_support());
        assert!(driver.text_support());
        assert!(driver.image_support());
        assert!(driver.multipage_support());
    }
}
