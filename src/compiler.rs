use crate::util::{self, EnvError};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Compiles the `input` resource (.rc) file and saves it to `output`.
pub fn compile<P, Q>(input: P, output: Q) -> Result<(), CompilerError>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let status = Command::new(compiler()?)
        .args([
            OsStr::new("/fo"),
            output.as_ref().as_os_str(),
            input.as_ref().as_os_str(),
        ])
        .status()
        .map_err(CompilerError::CommandFailed)?;

    if status.success() {
        Ok(())
    } else {
        Err(CompilerError::StatusFailure(status.code().unwrap()))
    }
}

fn compiler() -> Result<PathBuf, CompilerError> {
    let bin = sdk_bin_path()?;

    let mut tool_dirs: Vec<PathBuf> = std::fs::read_dir(bin)
        .map_err(CompilerError::SdkReadFailed)?
        .flatten()
        .filter(|dir| dir.file_type().is_ok_and(|ty| ty.is_dir()))
        .map(|dir| dir.path())
        .collect();

    // Sort so the latest version is first in the list
    tool_dirs.sort_by(|a, b| b.cmp(a));

    let arch = get_arch()?;
    for mut path in tool_dirs {
        path.push(arch);
        path.push("rc.exe");

        if path.is_file() {
            return Ok(path);
        }
    }

    Err(CompilerError::CompilerNotFound)
}

fn sdk_bin_path() -> Result<PathBuf, CompilerError> {
    let mut bin = sdk_path()?;
    bin.push("bin");
    Ok(bin)
}

fn sdk_path() -> Result<PathBuf, CompilerError> {
    Ok(PathBuf::from(
        windows_registry::LOCAL_MACHINE
            .open(r"SOFTWARE\Microsoft\Windows Kits\Installed Roots")
            .and_then(|key| key.get_string("KitsRoot10"))
            .map_err(|e| CompilerError::SdkNotFound(e.code().0))?,
    ))
}

fn get_arch() -> Result<&'static str, CompilerError> {
    let host = util::env_var("HOST")?;
    let arch = host
        .find('-')
        .map(|i| &host[..i])
        .ok_or(CompilerError::InvalidHost)?;

    match arch {
        "x86_64" => Ok("x64"),
        "i686" => Ok("x86"),
        "aarch64" => Ok("arm64"),
        _ => Err(CompilerError::UnsupportedArch),
    }
}

#[derive(Debug)]
pub enum CompilerError {
    EnvVarNotFound(EnvError),
    InvalidHost,
    UnsupportedArch,
    SdkNotFound(i32),
    SdkReadFailed(std::io::Error),
    CompilerNotFound,
    CommandFailed(std::io::Error),
    StatusFailure(i32),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvVarNotFound(e) => e.fmt(f),
            Self::InvalidHost => write!(f, "HOST env var is not a valid host triple."),
            Self::UnsupportedArch => write!(f, "Unsupported host architecture!"),
            Self::SdkNotFound(code) => write!(f, "Windows SDK not found! ({code:#10X})"),
            Self::SdkReadFailed(e) => write!(f, "Failed to read Windows SDK's bin directory. {e}"),
            Self::CompilerNotFound => write!(f, "Resource compiler (rc.exe) was not found!"),
            Self::CommandFailed(e) => write!(f, "Failed to execute the resource compiler. {e}"),
            Self::StatusFailure(code) => write!(f, "Failed to compile resource! Exit code: {code}"),
        }
    }
}

impl std::error::Error for CompilerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SdkReadFailed(error) => Some(error),
            Self::CommandFailed(error) => Some(error),
            _ => None,
        }
    }
}

impl From<EnvError> for CompilerError {
    fn from(error: EnvError) -> Self {
        Self::EnvVarNotFound(error)
    }
}
