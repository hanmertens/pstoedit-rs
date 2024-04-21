# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Features `pstoedit_4_00` and `pstoedit_4_01` to target pstoedit version 4.xx.
- Function `loadpstoeditplugins_plainC`, requiring feature `pstoedit_4_01`.
- Function `unloadpstoeditplugins`, requiring feature `pstoedit_4_00`.
- Field `formatGroup` in `DriverDescription_S`, requiring feature `pstoedit_4_00`.

## [0.1.0] &ndash; 2020-07-13
### Added
- Raw bindings to C API of [pstoedit](http://pstoedit.net), generated through
  [bindgen](https://github.com/rust-lang/rust-bindgen).

[Unreleased]: https://github.com/hanmertens/pstoedit-rs/compare/sys-v0.1.0...HEAD
[0.1.0]: https://github.com/hanmertens/pstoedit-rs/releases/tag/sys-v0.1.0
