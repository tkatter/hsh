use std::{fmt, io, error};
use crate::errors::ArgErr;

#[derive(Debug)]
pub enum HshErr {
    ArgumentErr(ArgErr),
    IoError(io::Error),
}

impl fmt::Display for HshErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArgumentErr(_) => write!(f, "Arg Error"),
            Self::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl error::Error for HshErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for HshErr {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
