use super::TraceOutputGenerator;

pub struct TraceTextGenerator;

impl TraceOutputGenerator for TraceTextGenerator {
    fn write<W: std::io::Write>(&self, dest: &mut W, trace: &crate::Trace) -> super::Result<()> {
        for entry in trace.entries.iter() {
            writeln!(dest, "{}", entry)?;
        }
        Ok(())
    }
}
