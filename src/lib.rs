use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error when executing `rustc -vV`. {0}")]
    Io(#[from] std::io::Error),
    #[error("Output of `rustc -vV` was not valid UTF-8. {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Unexpected output structure for `rustc -vV` after successful execution")]
    UnexpectedOutputStructure,
}

/// Returns the host triple of the current rustc using CLI.
///
/// Notice that such implementation relies on presence of `rustc` on the machine
/// where this function is called. However, it can be ran in a build script.
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

    let output_ref = std::str::from_utf8(&output.stdout).map_err(Error::from)?;

    match output_ref.lines().find_map(|l| l.strip_prefix("host: ")) {
        Some(host) => Ok(host.to_string()),
        None => Err(Error::UnexpectedOutputStructure),
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
