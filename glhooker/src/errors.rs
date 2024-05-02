use std::error;
use std::fmt;


#[derive(Debug)]
pub enum GLHookerError {
    InitError,
    RegisterHookError(String),
}

impl fmt::Display for GLHookerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GLHookerError::InitError => write!(f, "Could not initialize GLHooker"),
            GLHookerError::RegisterHookError(ref func) => {
                write!(f, "Could not hook function: {}", func)
            }
        }
    }
}

impl error::Error for GLHookerError {}
