#![doc = include_str!("../README.md")]

use std::process::Command;
use thiserror::Error;

/// Error type for `rustc_host`.
#[derive(Error, Debug)]
pub enum Error {
    /// I/O error when executing `rustc -vV`.
    #[error("I/O error when executing `rustc -vV`. {0}")]
    Io(#[from] std::io::Error),
    /// Output of `rustc -vV` was not valid UTF-8.
    #[error("Output of `rustc -vV` was not valid UTF-8. {0}")]
    Utf8(#[from] std::str::Utf8Error),
    /// Unexpected output structure for `rustc -vV` after successful execution.
    #[error("Unexpected output structure for `rustc -vV` after successful execution")]
    UnexpectedOutputStructure,
}

/// Returns the host triple of the current rustc using CLI.
///
/// Notice that such implementation relies on presence of `rustc` on the machine
/// where this function is called. Two good places for it are in a build script
/// or in a procedural macro.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/host.rs")]
/// ```
///
/// # Implementation details
///
/// *At the moment of writing*, it relies on the output of `rustc -vV`, which is expected to be
/// nearly in the following format:
///
/// ```text
/// rustc 1.66.0 (69f9c33d7 2022-12-12)
/// binary: rustc
/// commit-hash: 69f9c33d71c871fc16ac445211281c6e7a340943
/// commit-date: 2022-12-12
/// host: x86_64-pc-windows-msvc
/// release: 1.66.0
/// LLVM version: 15.0.2
/// ```
///
/// To be precise, it expects a line starting with `host: `.
pub fn from_cli() -> Result<String, Error> {
    let output = Command::new("rustc")
        .arg("-vV")
        .output()
        .map_err(Error::from)?;

    let stdout_buf =
        String::from_utf8(output.stdout).map_err(|e| Error::from(e.utf8_error()))?;
    // TODO: consider reusing the String from output
    #[cfg(not(feature = "unsafe"))]
    match stdout_buf.lines().find_map(|l| l.strip_prefix("host: ")) {
        Some(host) => Ok(host.to_string()),
        None => Err(Error::UnexpectedOutputStructure),
    }
    #[cfg(feature = "unsafe")]
    {
        const HOST_PREFIX: &str = "\nhost: ";
        let beg_incl = stdout_buf
            .find(HOST_PREFIX)
            .map(|i| i + HOST_PREFIX.len())
            .ok_or(Error::UnexpectedOutputStructure)?;

        let end_excl = unsafe { stdout_buf.get_unchecked(beg_incl..) }
            .find('\n')
            .map(|i| i + beg_incl)
            .ok_or(Error::UnexpectedOutputStructure)?;
        let mut bytes = stdout_buf.into_bytes();
        let src = unsafe { bytes.as_ptr().add(beg_incl) };
        let dst = bytes.as_mut_ptr();
        let count = end_excl - beg_incl;
        unsafe { std::ptr::copy(src, dst, count) };
        bytes.truncate(end_excl - beg_incl);
        Ok(unsafe { String::from_utf8_unchecked(bytes) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(all(
        target_arch = "x86_64",
        target_vendor = "pc",
        target_os = "windows",
        target_env = "msvc"
    ))]
    fn test_from_cli() {
        let host = from_cli().unwrap();
        assert_eq!(host, "x86_64-pc-windows-msvc");
    }
}
