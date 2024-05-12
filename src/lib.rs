mod errors;
pub mod generator;
mod hooks;
pub mod types;
pub mod trace;

pub use crate::generator::text_generator::TraceTextGenerator;
use glhooker::{GLHooker, HookDesc};
use hooks::get_hook;
use std::error::Error;
use trace::Trace;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct GLTracer {
    pub trace: Trace,
}


impl GLTracer {
    pub fn new() -> Result<Self> {
        GLHooker::init()?;
        let trace = Trace {
            entries: Box::new(Vec::new()),
        };
        Ok(Self { trace: trace })
    }
    pub fn trace_func(&mut self, symbol: &str) -> Result<()> {
        let hook = HookDesc::new(symbol, get_hook(symbol)?).with_userdata(&self.trace);
        GLHooker::register_hook(hook)
    }
}

#[cfg(test)]
mod tests {

    extern crate gl_loader;
    use crate::GLTracer;

    #[test]
    fn load_gl_symbol() -> Result<(), Box<dyn std::error::Error>> {
        let mut gltracer = GLTracer::new()?;
        gl_loader::init_gl();
        gl::load_with(|symbol| {
            if let Err(e) = gltracer.trace_func(symbol) {
                println!("{}", e);
            }
            gl_loader::get_proc_address(symbol) as *const _
        });
        Ok(())
    }
}
