# pstoedit

Rust bindings to [pstoedit](http://pstoedit.net/).

This crate contains Rust bindings to pstoedit, a C++ program that can translate
PostScript and PDF graphics into other vector formats.

## Requirements

Only dynamic linking to pstoedit is supported, so pstoedit needs to be
installed. The supported DLL version is 301, which is compatible with pstoedit
version 3.17 and higher.

## License

Licensed under the GNU General Public License; either version 2 of the License
([LICENSE](LICENSE) or https://www.gnu.org/licenses/old-licenses/gpl-2.0.html),
or (at your option) any later version.
