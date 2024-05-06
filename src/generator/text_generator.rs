use super::generator::{self, TraceOutputGenerator};

pub struct TraceTextGenerator;

impl TraceOutputGenerator for TraceTextGenerator {
    fn write<W: std::io::Write>(dest: &mut W) -> generator::Result<()> {
        write!(dest, "Hello")?;
        Ok(())
    }
}
