pub mod html_generator;
pub mod text_generator;

use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub trait TraceOutputGenerator {
    fn write<W: std::io::Write>(&self, dest: &mut W, trace: &crate::Trace) -> Result<()>;
}
