use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GLHookerRegisterHookDesc {
    pub src_func_name: [u8; 64],
    pub dst_func: *mut c_void,
    pub userdata_size: usize,
    pub userdata: *const c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct InternalHook {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type HookHandle = *mut InternalHook;

#[link(name = "glhooker")]
extern "C" {
    pub fn glhooker_init() -> bool;
    pub fn glhooker_registerhook(desc: *const GLHookerRegisterHookDesc) -> bool;
    pub fn glhooker_gethookname(handle: HookHandle) -> *const i8;
    pub fn glhooker_gethook(name: *const u8) -> HookHandle;
    pub fn glhooker_getoriginalfunction(handle: HookHandle) -> *mut c_void;
    pub fn glhooker_gethookuserdata(handle: HookHandle) -> *const c_void;
}
