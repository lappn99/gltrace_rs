use crate::generator;


pub mod enums;
use enums::TraceEntryParamValue;

#[derive(Debug, Clone)]
pub struct TraceParam(pub String,  pub TraceEntryParamValue);

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub function: String,
    pub params: Vec<TraceParam>

}

pub struct Trace {
    pub entries: Box<Vec<TraceEntry>>,
}

impl Trace {
    pub fn write_trace<W, G>(&mut self, trace_generator: G, writer: &mut W) -> crate::Result<()>
    where
        W: std::io::Write,
        G: generator::TraceOutputGenerator,
    {
        trace_generator.write(writer, self)
    }
}

impl TraceEntry {
    pub fn new(function: &str) -> Self {
        Self {
            function: String::from(function),
            params: Vec::new()
        }
    }

    pub fn with_param<T>(&mut self, name: Option<&str>, value: Option<TraceEntryParamValue>) -> &Self {
        if name.is_some() && value.is_some() {
            self.params.push(TraceParam(String::from(name.unwrap()),TraceEntryParamValue::from(value.unwrap())));
        }
        
        self
    }
}

