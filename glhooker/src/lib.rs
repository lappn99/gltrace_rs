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
    glhooker_gethook, glhooker_gethookname, glhooker_gethookuserdata, glhooker_getoriginalfunction,
    glhooker_init, GLHookerHookType, GLHookerRegisterHookDesc, HookHandle,
};

pub enum HookType {
    Inline,
    Intercept,
}

pub struct GLHooker;

#[derive(Debug, Clone, Copy)]
pub struct Hook {
    internal_hook: HookHandle,
}

pub struct HookDesc<'a, T>
where
    T: Sized,
{
    pub hook_type: HookType,
    pub source_func_name: &'a str,
    pub dst_func: *mut c_void,
    pub userdata: Option<&'a T>,
}

impl<'a, T> HookDesc<'a, T> {
    pub fn new(hook_type: HookType, source_name: &'a str, dest_func: *mut c_void) -> Self {
        HookDesc {
            hook_type: hook_type,
            source_func_name: source_name,
            dst_func: dest_func,
            userdata: None,
        }
    }

    pub fn with_userdata(mut self, userdata: &'a T) -> Self {
        self.userdata = Some(userdata);
        self
    }
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

    pub fn register_hook<'a, T>(hook: HookDesc<'a, T>) -> Result<()> {
        unsafe {
            let mut desc = wrapper::GLHookerRegisterHookDesc {
                hook_type: match hook.hook_type {
                    HookType::Inline => GLHookerHookType::Inline,
                    HookType::Intercept => GLHookerHookType::Intercept,
                },
                src_func_name: [0; 64],
                dst_func: hook.dst_func,
                userdata_size: if hook.userdata.is_some() {
                    std::mem::size_of::<T>()
                } else {
                    0
                },
                userdata: if let Some(userdata) = hook.userdata {
                    std::ptr::from_ref(userdata) as *const c_void
                } else {
                    std::ptr::null()
                },
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

    pub fn get_hook<'a>(name: &'a str) -> Result<Hook> {
        let handle: HookHandle = unsafe {
            let name = CString::new(name)?;
            glhooker_gethook(name.as_bytes().as_ptr())
        };
        if handle == std::ptr::null_mut() {
            Err(GLHookerError::GetHookError(String::from(name)).into())
        } else {
            Ok(Hook {
                internal_hook: handle,
            })
        }
    }
}

impl Hook {
    pub fn get_name(&self) -> Result<&'static str> {
        let name = unsafe {
            let name = glhooker_gethookname(self.internal_hook);
            CStr::from_ptr(name)
        };
        if let Ok(name) = name.to_str() {
            return Ok(name);
        } else {
            Err(GLHookerError::GetHookNameError.into())
        }
    }

    pub fn get_target_function(&self) -> Result<*mut c_void> {
        let func = unsafe { glhooker_getoriginalfunction(self.internal_hook) };

        if func == std::ptr::null_mut() {
            Err(GLHookerError::GetTargetFunctionError.into())
        } else {
            Ok(func)
        }
    }

    pub fn get_userdata<T>(&self) -> Result<&'static T>
    where
        T: Sized,
    {
        let userdata = unsafe {
            let userdata = glhooker_gethookuserdata(self.internal_hook) as *const T;
            userdata.as_ref()
        };

        if let Some(userdata) = userdata {
            Ok(userdata)
        } else {
            Err(GLHookerError::GetUserDataError.into())
        }
    }

    pub fn get_userdata_mut<T>(&self) -> Result<&'static mut T> {
        let userdata = unsafe {
            let userdata = glhooker_gethookuserdata(self.internal_hook) as *mut T;
            userdata.as_mut()
        };

        if let Some(userdata) = userdata {
            Ok(userdata)
        } else {
            Err(GLHookerError::GetUserDataError.into())
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{errors::GLHookerError, GLHooker, HookDesc, HookType};
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
        let hook = HookDesc::<()>::new(HookType::Intercept, "glBindBuffer", hook as *mut c_void);

        assert_eq!(GLHooker::register_hook(hook)?, ());
        Ok(())
    }

    #[test]
    pub fn test_load_one() -> Result<()> {
        GLHooker::init()?;
        gl_loader::init_gl();
        let hook: HookDesc<_> =
            HookDesc::<()>::new(HookType::Intercept, "glBindBuffer", hook as *mut c_void);

        GLHooker::register_hook(hook)?;
        gl::load_with(|f| gl_loader::get_proc_address(f) as *const _);
        Ok(())
    }

    #[test]
    pub fn test_get_hook() -> Result<()> {
        GLHooker::init()?;
        let hook: HookDesc<_> =
            HookDesc::<()>::new(HookType::Intercept, "glBindBuffer", hook as *mut c_void);

        GLHooker::register_hook(hook)?;
        let _ = GLHooker::get_hook("glBindBuffer").unwrap();

        Ok(())
    }

    #[test]
    pub fn test_get_bad_hook() -> Result<()> {
        GLHooker::init()?;
        let hook: HookDesc<_> =
            HookDesc::<()>::new(HookType::Intercept, "glBindBuffer", hook as *mut c_void);

        GLHooker::register_hook(hook)?;
        let hook = GLHooker::get_hook("glBinduffer");
        assert!(hook.unwrap_err().is::<GLHookerError>());

        Ok(())
    }

    #[test]
    pub fn test_get_hook_name() -> Result<()> {
        GLHooker::init()?;
        let hook: HookDesc<_> =
            HookDesc::<()>::new(HookType::Intercept, "glBindBuffer", hook as *mut c_void);

        GLHooker::register_hook(hook)?;
        let hook = GLHooker::get_hook("glBindBuffer").unwrap();
        assert_eq!(hook.get_name().unwrap(), "glBindBuffer");
        Ok(())
    }

    #[test]
    pub fn test_get_userdata() -> Result<()> {
        GLHooker::init()?;
        let userdata = 69;

        let hook: HookDesc<_> =
            HookDesc::<i32>::new(HookType::Intercept, "glBindBuffer", hook as *mut c_void)
                .with_userdata(&userdata);
        GLHooker::register_hook(hook)?;
        let hook = GLHooker::get_hook("glBindBuffer").unwrap();
        let userdata = hook.get_userdata::<i32>().unwrap();
        assert_eq!(*userdata, 69);
        Ok(())
    }
}
