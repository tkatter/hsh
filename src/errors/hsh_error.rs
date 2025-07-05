use crate::errors::ArgErr;

#[derive(Debug)]
pub enum HshErr {
    ArgumentErr(ArgErr),
    // ProcessErr(ProcErr),
}
