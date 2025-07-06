use std::fmt;

#[derive(Debug)]
pub struct ArgErr {
    exit_code: u8,
    err_type: ArgErrType,
    message: String,
}

#[derive(Debug)]
pub enum ArgErrType {
    UnknownArg,
    SomeOtherType,
}

impl fmt::Display for ArgErrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SomeOtherType => write!(f, "Some other type"),
            Self::UnknownArg => write!(f, "Unknown argument"),
        }
    }
}
