use std::error;


pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub trait TraceOutputGenerator {
    
    fn write<W: std::io::Write> (dest: &mut W) -> Result<()>;
}