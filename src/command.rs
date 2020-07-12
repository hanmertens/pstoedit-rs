use crate::{smallvec, Result, SmallVec};
use std::ffi::CString;

/// Command builder for generic pstoedit interaction.
///
/// Commands are the main way to interact with pstoedit. A command is typically
/// constructed using [`arg`][Command::arg], [`args`][Command::args] and/or
/// [`args_slice`][Command::args_slice]. It can be run using
/// [`run`][Command::run], multiple times if necessary.
///
/// # Examples
/// ```
/// use pstoedit::Command;
///
/// pstoedit::init()?;
/// Command::new().arg("-gstest")?.run()?;
/// # Ok::<(), pstoedit::Error>(())
/// ```
///
/// ```no_run
/// use pstoedit::Command;
///
/// pstoedit::init()?;
/// Command::new().args_slice(&["-f", "latex2e", "input.ps", "output.tex"])?.run()?;
/// # Ok::<(), pstoedit::Error>(())
/// ```
///
/// # Errors
/// Most methods can raise [`NulError`][crate::Error::NulError] if a passed
/// string contains an internal nul byte. Only [`run`][Command::run] can raise
/// different errors.
#[derive(Clone, Debug)]
pub struct Command {
    args: SmallVec<CString>,
    gs: Option<CString>,
}

impl Command {
    /// Create a command with program name and without arguments.
    ///
    /// The program name is already set in this function and should not be set
    /// using [`arg`][Command::arg], [`args`][Command::args], or
    /// [`args_slice`][Command::args_slice].
    pub fn new() -> Self {
        Self {
            args: smallvec![CString::new("pstoedit").unwrap()],
            gs: None,
        }
    }

    /// Add a single argument.
    ///
    /// For more information, examples, and errors, see [`Command`].
    pub fn arg<S>(&mut self, arg: S) -> Result<&mut Self>
    where
        S: Into<Vec<u8>>,
    {
        self.args.push(CString::new(arg.into())?);
        Ok(self)
    }

    /// Add multiple arguments.
    ///
    /// # Examples
    /// ```no_run
    /// use pstoedit::Command;
    ///
    /// pstoedit::init()?;
    /// Command::new().args(std::env::args().skip(1))?.run()?;
    /// # Ok::<(), pstoedit::Error>(())
    /// ```
    ///
    /// # Errors
    /// [`NulError`][crate::Error::NulError] if a passed string contains an
    /// internal nul byte. Only the arguments before this string will have been
    /// added. Ownership of these later arguments will not be returned, consider
    /// using [`arg`][Command::arg] if necessary for more control.
    pub fn args<I>(&mut self, args: I) -> Result<&mut Self>
    where
        I: IntoIterator,
        I::Item: Into<Vec<u8>>,
    {
        for arg in args {
            self.arg(arg.into())?;
        }
        Ok(self)
    }

    /// Add multiple arguments from slice.
    ///
    /// Ownership of arguments is not passed for ergonomic reasons. If the
    /// potential optimization benefits of passing ownership are desired, use
    /// [`args`][Command::args] or [`arg`][Command::arg]. This can be the case
    /// when passing instances of [`String`] or [`Vec<u8>`][Vec].
    ///
    /// # Examples
    /// See [`Command`][Command#examples].
    ///
    /// # Errors
    /// [`NulError`][crate::Error::NulError] if a passed string contains an
    /// internal nul byte. Only the arguments before this string will have been
    /// added.
    pub fn args_slice<S>(&mut self, args: &[S]) -> Result<&mut Self>
    where
        S: AsRef<str>,
    {
        for arg in args {
            self.arg(arg.as_ref())?;
        }
        Ok(self)
    }

    /// Specify ghostscript executable.
    ///
    /// By default pstoedit tries to automatically determine this value. The
    /// specifics of this are platform-dependent.
    ///
    /// # Examples
    /// ```no_run
    /// use pstoedit::Command;
    ///
    /// pstoedit::init()?;
    /// // Use personal ghostscript executable that is not in PATH
    /// let gs = "/home/user/projects/ghostscript/bin/gs";
    /// Command::new().arg("-gstest")?.gs(gs)?.run()?;
    /// # Ok::<(), pstoedit::Error>(())
    /// ```
    pub fn gs<S>(&mut self, gs: S) -> Result<&mut Self>
    where
        S: Into<Vec<u8>>,
    {
        self.gs = Some(CString::new(gs.into())?);
        Ok(self)
    }

    /// Run the command.
    ///
    /// This can be done multiple times for the same [`Command`].
    ///
    /// # Examples
    /// See [`Command`][Command#examples].
    ///
    /// # Errors
    /// - [`NotInitialized`][crate::Error::NotInitialized] if
    /// [`init`][crate::init] was not called successfully.
    /// - [`PstoeditError`][crate::Error::PstoeditError] if pstoedit returns
    /// with a non-zero status code.
    pub fn run(&self) -> Result<()> {
        crate::pstoedit_cstr(&self.args, self.gs.as_ref())
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn prep() {
        crate::init().unwrap();
        // Ensure ghostscript is not obtained through environment
        env::set_var("GS", "should_not_be_used");
    }

    #[test]
    fn arg_gs() {
        prep();
        Command::new()
            .arg("-gstest")
            .unwrap()
            .gs("gs")
            .unwrap()
            .run()
            .unwrap();
    }

    #[test]
    fn args_gs() {
        prep();
        Command::new()
            .args_slice(&["-gstest"])
            .unwrap()
            .gs("gs")
            .unwrap()
            .run()
            .unwrap();
    }
}
