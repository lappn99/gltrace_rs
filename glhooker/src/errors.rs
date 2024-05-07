use std::error;
use std::fmt;

#[derive(Debug)]
pub enum GLHookerError {
    InitError,
    RegisterHookError(String),
    GetOriginalNameError,
    GetHookError(String),
    GetHookNameError,
    GetTargetFunctionError,
    GetUserDataError,
}

impl fmt::Display for GLHookerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GLHookerError::InitError => write!(f, "Could not initialize GLHooker"),
            GLHookerError::RegisterHookError(ref func) => {
                write!(f, "Could not hook function: {}", func)
            }
            GLHookerError::GetOriginalNameError => {
                write!(f, "Could not get original name")
            }
            GLHookerError::GetHookError(ref name) => {
                write!(f, "Could not get hook: {}", name)
            }
            GLHookerError::GetHookNameError => {
                write!(f, "Could not get hook name")
            }
            GLHookerError::GetTargetFunctionError => {
                write!(f, "Could not get target function pointer for hook")
            }
            GLHookerError::GetUserDataError => {
                write!(f, "Could not get user data for hook")
            }
        }
    }
}

impl error::Error for GLHookerError {}
