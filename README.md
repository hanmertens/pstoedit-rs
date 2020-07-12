# pstoedit

![CI](https://github.com/hanmertens/pstoedit-rs/workflows/CI/badge.svg)

Rust bindings to [pstoedit](http://pstoedit.net).

This crate contains Rust bindings to pstoedit, a C++ program that can translate
PostScript and PDF graphics into other vector formats.

## Features

The API is similar to the C API of pstoedit. Information on drivers can be
inquired and arbitrary commands can be passed to pstoedit via the `pstoedit`
function.

Optional Cargo features:
- `smallvec`: potentially reduce the number of allocations using the
  [`smallvec`](https://crates.io/crates/smallvec) crate.

## Examples

```rust
pstoedit::init().unwrap();

// For every driver ...
for driver in &pstoedit::DriverInfo::get().unwrap() {
    let format = driver.symbolic_name().unwrap();
    let extension = driver.extension().unwrap();
    let output_name = format!("output-{}.{}", format, extension);

    // ... convert input.ps to that format
    let cmd = ["pstoedit", "-f", format, "input.ps", output_name.as_ref()];
    pstoedit::pstoedit(&cmd, None).unwrap();
}
```

## Requirements

Only dynamic linking to pstoedit is supported, so pstoedit needs to be
installed. The supported DLL version is 301, which is compatible with pstoedit
version 3.17 and higher.

## License

Licensed under the GNU General Public License; either version 2 of the License
([LICENSE](LICENSE) or https://www.gnu.org/licenses/old-licenses/gpl-2.0.html),
or (at your option) any later version.
