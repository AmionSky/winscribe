use crate::compiler::CompilerError;
use crate::util::EnvError;

#[derive(Debug)]
pub enum ResError {
    Io(std::io::Error),
    Compiler(CompilerError),
    EnvVarNotFound(EnvError),
    Custom(&'static str),
}

impl std::fmt::Display for ResError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::Compiler(error) => error.fmt(f),
            Self::EnvVarNotFound(error) => error.fmt(f),
            Self::Custom(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for ResError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<std::io::Error> for ResError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<CompilerError> for ResError {
    fn from(error: CompilerError) -> Self {
        Self::Compiler(error)
    }
}

impl From<EnvError> for ResError {
    fn from(error: EnvError) -> Self {
        Self::EnvVarNotFound(error)
    }
}
