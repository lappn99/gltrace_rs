use std::{
    os::raw::c_void,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GLHookerHookType {
    Inline,
    Intercept,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GLHookerRegisterHookDesc {
    pub hook_type: GLHookerHookType,
    pub src_func_name: [u8;64],
    pub dst_func: *mut c_void,
}

#[link(name = "glhooker")]
extern "C" {

    pub fn glhooker_init() -> bool;
    pub fn glhooker_registerhook(desc: *const GLHookerRegisterHookDesc) -> bool;

}
