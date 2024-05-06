use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GLTraceError {
    HookNotFound(String),
}

impl fmt::Display for GLTraceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GLTraceError::HookNotFound(ref symbol) => {
                write!(f, "Hook for symbol \"{}\" not found", symbol)
            }
        }
    }
}

impl error::Error for GLTraceError {}
