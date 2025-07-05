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
