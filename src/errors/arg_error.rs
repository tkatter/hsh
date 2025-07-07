use std::fmt;

#[derive(Debug)]
pub struct ArgErr {
    pub exit_code: u8,
    pub err_type: ArgErrType,
    pub message: String,
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
