# pstoedit

[![CI](https://github.com/hanmertens/pstoedit-rs/workflows/CI/badge.svg)](https://github.com/hanmertens/pstoedit-rs/actions?query=workflow%3ACI)

Rust bindings to [pstoedit](http://pstoedit.net).

This crate contains Rust bindings to pstoedit, a C++ program that can translate
PostScript and PDF graphics into other vector formats.

## Features

The API is similar to the C API of pstoedit. Information on drivers can be
inquired and arbitrary commands to pstoedit can be constructed and run.

Optional Cargo features:
- `smallvec`: potentially reduce the number of allocations using the
  [`smallvec`](https://crates.io/crates/smallvec) crate.

## Examples

```rust
use pstoedit::{DriverInfo, Command};

pstoedit::init()?;

// For every driver ...
for driver in &DriverInfo::get()? {
    let format = driver.symbolic_name()?;
    let extension = driver.extension()?;
    let output_name = format!("output-{}.{}", format, extension);

    // ... convert input.ps to that format
    Command::new().args_slice(&["-f", format, "input.ps"])?.arg(output_name)?.run()?;
}
```

## Requirements

Only dynamic linking to pstoedit is supported, so pstoedit needs to be
installed. The supported DLL version is 301, which is compatible with pstoedit
version 3.xx starting from 3.17. Currently, pstoedit version 4.00 and higher
are not supported.

## License

Licensed under the GNU General Public License; either version 2 of the License
([LICENSE](LICENSE) or https://www.gnu.org/licenses/old-licenses/gpl-2.0.html),
or (at your option) any later version.
