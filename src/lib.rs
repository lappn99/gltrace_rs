mod errors;
pub mod generator;
mod hooks;
pub mod macros;
pub mod types;

pub use crate::generator::text_generator::TraceTextGenerator;
use glhooker::{GLHooker, HookDesc};
use hooks::get_hook;
use std::{any::TypeId, collections::HashMap, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;



pub struct GLTracer {
    pub trace: Trace,
}

pub struct Trace {
    pub entries: Box<Vec<String>>,
}

pub struct TraceEntry {
    proc: String,
    args: HashMap<String, TypeId>

}

impl Trace {
    pub fn write_trace<W, G>(&mut self, trace_generator: G, writer: &mut W) -> Result<()>
    where
        W: std::io::Write,
        G: generator::TraceOutputGenerator,
    {
        trace_generator.write(writer, self)
    }
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
        let hook =
            HookDesc::new(symbol, get_hook(symbol)?).with_userdata(&self.trace);
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
                println!("{}",e);
            }
            gl_loader::get_proc_address(symbol) as *const _
        });
        Ok(())
    }
}
