use crate::ResError;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct EnvError(&'static str);

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} environment varibale not found! Is it called from a build script?",
            self.0
        )
    }
}

impl std::error::Error for EnvError {}

/// Gets the `OUT_DIR` environment variable as a path.
pub(crate) fn out_dir() -> Result<PathBuf, EnvError> {
    Ok(PathBuf::from(env_var_os("OUT_DIR")?))
}

/// Path to a file in the build's `OUT_DIR`.
pub(crate) fn out_file<P: AsRef<Path>>(file: P) -> Result<PathBuf, EnvError> {
    let mut path = out_dir()?;
    path.push(file);
    Ok(path)
}

/// Gets the environment varibale.
pub(crate) fn env_var(var: &'static str) -> Result<String, EnvError> {
    std::env::var(var).map_err(|_| EnvError(var))
}

/// Gets the environment varibale as an OsString.
pub(crate) fn env_var_os(var: &'static str) -> Result<OsString, EnvError> {
    std::env::var_os(var).ok_or(EnvError(var))
}

/// Write byte slice to a new file
pub(crate) fn to_file<P: AsRef<Path>>(path: P, data: &[u8]) -> std::io::Result<()> {
    File::create(path)?.write_all(data)
}

// Copied from tauri-winres
/// Escape string for use in the resource file.
pub(crate) fn escape(string: &str) -> String {
    let mut escaped = String::new();
    for chr in string.chars() {
        // In quoted RC strings, double-quotes are escaped by using two
        // consecutive double-quotes.  Other characters are escaped in the
        // usual C way using backslashes.
        match chr {
            '"' => escaped.push_str("\"\""),
            '\'' => escaped.push_str("\\'"),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            _ => escaped.push(chr),
        };
    }
    escaped
}

pub(crate) fn escape_path<P: AsRef<Path>>(path: P) -> Result<String, ResError> {
    Ok(escape(path.as_ref().canonicalize()?.to_str().ok_or(
        ResError::Custom("Failed to convert path to string! Not a valid UTF-8?"),
    )?))
}
