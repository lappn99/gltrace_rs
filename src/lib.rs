use std::error::Error;

use glhooker::{GLHooker, Hook, HookType};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct GLTrace;

impl GLTrace {
    pub fn init() -> Result<()> {
        let _ = GLHooker::init()?;
        let _hook: Hook = Hook {
            hook_type: HookType::Intercept,
            source_func_name: "glBindBuffer",
            dst_func: trace,
        };
        let _ = GLHooker::register_hook_all(HookType::Intercept, trace);
        return Ok(());
    }
}

#[no_mangle]
pub unsafe extern "C" fn trace() {
    println!("Works!");
}

#[cfg(test)]
mod tests {

    use crate::GLTrace;
    
    use std::error;
    #[test]
    pub fn test_init() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(GLTrace::init()?, ());
        Ok(())
    }
}
