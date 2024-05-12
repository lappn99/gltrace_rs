use super::TraceOutputGenerator;

pub struct TraceTextGenerator;

impl TraceOutputGenerator for TraceTextGenerator {
    fn write<W: std::io::Write>(&self, dest: &mut W, trace: &crate::Trace) -> super::Result<()> {
        for entry in trace.entries.iter() {
            write!(dest, "gl{}(", entry.function )?;
            for param in entry.params.iter() {
                write!(dest,"{}:{:?}, ", param.0,param.1)?;
            }
            writeln!(dest,")")?;
        }
        Ok(())
    }
}
