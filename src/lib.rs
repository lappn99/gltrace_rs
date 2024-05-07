mod errors;
pub mod generator;
mod hooks;
pub mod macros;
pub mod types;

use gl_loader;
use glhooker::{GLHooker, HookDesc, HookType};
use hooks::get_hook;
use std::{error::Error, fs};
pub use crate::generator::text_generator::TraceTextGenerator;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct GLTrace {
    pub trace: Trace
}

pub struct Trace {
    pub entries: Box<Vec<String>>
}

impl Trace {
    pub fn write_trace<W, G>(&mut self,trace_generator: G, writer: &mut W) -> Result<()>
    where 
        W: std::io::Write,
        G: generator::TraceOutputGenerator
    {
        trace_generator.write(writer, self)
    }
}

impl GLTrace {

    pub fn new() -> Result<Self> {
        GLHooker::init()?;
        let trace = Trace {
            entries: Box::new(Vec::new())
        };
        
        Ok(Self {
            trace: trace
        })
    }

    
    pub fn trace_func(&mut self,symbol: &str) -> Result<()> {
        //self.trace.entries.push(format!("Hooking {}", symbol));
        let hook = HookDesc::new(HookType::Inline, symbol, get_hook(symbol)?).with_userdata(&self.trace);
        GLHooker::register_hook(hook)
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

    
}
