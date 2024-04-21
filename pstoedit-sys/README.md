# pstoedit-sys

[![CI sys](https://github.com/hanmertens/pstoedit-rs/workflows/CI%20sys/badge.svg)](https://github.com/hanmertens/pstoedit-rs/actions?query=workflow%3A"CI+sys")

Native bindings to [pstoedit](http://pstoedit.net).

This crate contains low-level bindings to the C API of pstoedit, a C++ program
that can translate PostScript and PDF graphics into other vector formats.

## Requirements and compatibility

Only dynamic linking to pstoedit is supported, so pstoedit needs to be
installed. Multiple versions are supported, but the appropriate feature starting
with `pstoedit_` has to be enabled to prevent a runtime error. If multiple are
specified, the first in the following list takes precedence.

- `pstoedit_4_01`: compatible with pstoedit version 4.01, and likely with future
  4.xx releases.
- `pstoedit_4_00`: compatible with pstoedit version 4.00&ndash;4.01, and likely
  with future 4.xx releases.
- No feature starting with `pstoedit_`: compatible with pstoedit version
  3.17&ndash;3.78.

## License

Licensed under the GNU General Public License; either version 2 of the License
([LICENSE](LICENSE) or https://www.gnu.org/licenses/old-licenses/gpl-2.0.html),
or (at your option) any later version.
