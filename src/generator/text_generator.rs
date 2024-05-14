use super::TraceOutputGenerator;
use chrono::{DateTime, Utc};

pub struct TraceTextGenerator;

impl TraceOutputGenerator for TraceTextGenerator {
    fn write<W: std::io::Write>(&self, dest: &mut W, trace: &crate::Trace) -> super::Result<()> {
        let dt: DateTime<Utc> = trace.start_time.clone().into();
        writeln!(dest, "Trace start: {}", dt.format("%+"))?;
        for entry in trace.entries.iter() {
            if let Some(entry_time) = entry.time_start {
                write!(
                    dest,
                    "[+{}ms] gl{}(",
                    entry_time.duration_since(trace.start_time)?.as_millis(),
                    entry.function
                )?;
            } else {
                write!(dest, "gl{}(", entry.function)?;
            }

            if let Some(params) = &entry.params {
                write!(
                    dest,
                    "{}",
                    params
                        .into_iter()
                        .map(|param| format!("{}: {:?}", param.0, param.1))
                        .collect::<Vec<String>>()
                        .join(",")
                )?;
            }

            write!(dest, ")")?;

            if let Some(entry_start) = entry.time_start {
                if let Some(entry_end) = entry.time_end {
                    writeln!(
                        dest,
                        " Duration: {}µs ",
                        entry_end.duration_since(entry_start)?.as_micros()
                    )?;
                }
            }
        }
        Ok(())
    }
}
