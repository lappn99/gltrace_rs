#![feature(fn_traits)]
#![feature(tuple_trait)]
pub mod errors;

mod wrapper;

use std::{
    error,
    ffi::{c_void, CStr, CString},
};

use errors::GLHookerError;
use wrapper::{
    glhooker_getoriginalfunction, glhooker_init, GLHookerHookType, GLHookerRegisterHookDesc,
};

pub struct GLHooker {}

pub enum HookType {
    Inline,
    Intercept,
}

pub struct Hook<'a> {
    pub hook_type: HookType,
    pub source_func_name: &'a str,
    pub dst_func: *mut c_void,
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

impl GLHooker {
    pub fn init() -> Result<()> {
        unsafe {
            if !glhooker_init() {
                Err((GLHookerError::InitError).into())
            } else {
                Ok(())
            }
        }
    }

    pub fn register_hook<'a>(hook: Hook<'a>) -> Result<()> {
        let _src_name: CString = CString::new(hook.source_func_name)?;
        unsafe {
            let mut desc = wrapper::GLHookerRegisterHookDesc {
                hook_type: match hook.hook_type {
                    HookType::Inline => GLHookerHookType::Inline,
                    HookType::Intercept => GLHookerHookType::Intercept,
                },
                src_func_name: [0; 64],
                dst_func: hook.dst_func,
            };

            let n = std::cmp::min(hook.source_func_name.len(), 64);
            desc.src_func_name[0..n].copy_from_slice(&hook.source_func_name.as_bytes()[0..n]);

            if !wrapper::glhooker_registerhook(&desc as *const GLHookerRegisterHookDesc) {
                Err(GLHookerError::RegisterHookError(String::from(hook.source_func_name)).into())
            } else {
                Ok(())
            }
        }
    }

    pub fn register_hook_all<'a>(hook_type: HookType, func: *mut c_void) -> Result<()> {
        GLHooker::register_hook(Hook {
            hook_type: hook_type,
            source_func_name: &String::new(),
            dst_func: func,
        })
    }

    pub fn get_original_name(addr: *const c_void) -> Result<&'static str> {
        unsafe {
            let result = wrapper::glhooker_getoriginalname(addr);
            if result.is_null() {
                Err(GLHookerError::GetOriginalNameError.into())
            } else {
                Ok(CStr::from_ptr(result).to_str()?)
            }
        }
    }

    pub fn get_original_function() -> Result<*mut c_void> {
        let addr: *mut c_void = unsafe { glhooker_getoriginalfunction() };
        if addr.is_null() {
            Err(GLHookerError::GetOriginalFunctionError.into())
        } else {
            Ok(addr)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{GLHooker, Hook, HookType};
    use core::ffi::c_void;
    use gl;
    use std::error;

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[no_mangle]
    pub unsafe extern "C" fn hook() {
        println!("Works!");
    }

    #[test]
    pub fn test_init() -> Result<()> {
        assert_eq!(GLHooker::init()?, ());
        Ok(())
    }

    #[test]
    pub fn test_register_hook_one() -> Result<()> {
        GLHooker::init()?;
        gl_loader::init_gl();
        let hook: Hook = Hook {
            hook_type: HookType::Intercept,
            source_func_name: "glBindBuffer",
            dst_func: hook as *mut c_void,
        };

        assert_eq!(GLHooker::register_hook(hook)?, ());
        Ok(())
    }

    #[test]
    pub fn test_register_hook_all() -> Result<()> {
        GLHooker::init()?;
        gl_loader::init_gl();

        assert_eq!(
            GLHooker::register_hook_all(HookType::Intercept, hook as *mut c_void)?,
            ()
        );
        Ok(())
    }

    #[test]
    pub fn test_load_one() -> Result<()> {
        GLHooker::init()?;
        gl_loader::init_gl();
        let hook: Hook = Hook {
            hook_type: HookType::Intercept,
            source_func_name: "glBindBuffer",
            dst_func: hook as *mut c_void,
        };

        GLHooker::register_hook(hook)?;
        gl::load_with(|f| gl_loader::get_proc_address(f) as *const _);
        Ok(())
    }
}
