#![feature(fn_traits)]
mod errors;
pub mod generator;
mod hooks;
pub mod macros;
pub mod types;

use gl_loader;
use glhooker::{GLHooker, Hook, HookType};
use hooks::get_hook;
use std::error::Error;

#[macro_use]
extern crate lazy_static;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/*
lazy_static! {
    static ref TRACE_OUT: Mutex< dyn std::io::Write + Send> = Mutex::new(std::io::stdout());
}
 */
pub struct GLTrace;

impl GLTrace {
    pub fn init() -> Result<()> {
        GLHooker::init()?;
        Ok(())
    }
    pub fn trace_func(symbol: &str) -> Result<()> {
        GLHooker::register_hook(Hook {
            hook_type: HookType::Inline,
            source_func_name: symbol,
            dst_func: get_hook(symbol)?,
        })
    }

    pub fn trace_call() -> Result<()> {
        Ok(())
    }

    pub fn get_gl_func(str: &str) -> *const () {
        gl_loader::get_proc_address(str)
    }
}

#[no_mangle]
pub unsafe extern "C" fn trace() {
    println!("Trace");
}

#[cfg(test)]
mod tests {

    use crate::{types::types::GLbitfield, GLTrace};

    use gl;
    use gl_loader;
    use glhooker::errors::GLHookerError;
    use std::{error, io::ErrorKind};
    #[test]
    pub fn test_new() -> Result<(), Box<dyn error::Error>> {
        let _ = GLTrace::init()?;
        Ok(())
    }

    #[test]
    pub fn test_tracefunc() -> Result<(), Box<dyn error::Error>> {
        let trace = GLTrace::init()?;
        gl_loader::init_gl();
        gl::load_with(|symbol| {
            match GLTrace::trace_func(symbol) {
                Err(error) => println!("{:?}", error),
                _ => (),
            }
            gl_loader::get_proc_address(symbol) as *const _
        });

        Ok(())
    }

    #[test]
    pub fn test_tracefuncfail() -> Result<(), Box<dyn error::Error>> {
        let _ = GLTrace::init()?;
        let e = GLTrace::trace_func("glBinBuffer");
        assert!(e.is_err());
        assert_eq!(
            format!("{}", e.unwrap_err()),
            "Hook for symbol \"glBinBuffer\" not found"
        );
        Ok(())
    }

    #[test]
    pub fn test_noinit() -> Result<(), Box<dyn error::Error>> {
        let e = GLTrace::trace_func("glBindBuffer");
        assert!(e.is_err());
        assert!(e.unwrap_err().is::<GLHookerError>());
        Ok(())
    }

    #[no_mangle]
    pub fn gl_clear(mask: GLbitfield) {
        let _ = GLTrace::trace_call(/* Stuff */);
    }
}
