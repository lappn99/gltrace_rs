use crate::generator;

#[cfg(feature = "gpu_queries")]
use crate::gpu_query::enums::QueryTarget;

pub mod enums;
pub mod resource;
#[cfg(feature = "gpu_queries")]
use crate::gpu_query::QueryObject;
use enums::TraceEntryParamValue;
use std::rc::Rc;
use std::time;

use self::resource::ResourceTransaction;

#[derive(Debug, Clone)]
pub struct TraceParam(pub String, pub TraceEntryParamValue);

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub function: String,
    pub params: Option<Vec<TraceParam>>,
    pub time_start: Option<time::SystemTime>,
    pub time_end: Option<time::SystemTime>,
}

pub struct Trace {
    pub entries: Box<Vec<Rc<TraceEntry>>>,
    pub start_time: time::SystemTime,
    #[cfg(feature = "gpu_queries")]
    pub query_object: Option<QueryObject>,
    pub resource_transactions: Box<Vec<ResourceTransaction>>,
}

impl Trace {
    pub fn write_trace<W, G>(&mut self, trace_generator: G, writer: &mut W) -> crate::Result<()>
    where
        W: std::io::Write,
        G: generator::TraceOutputGenerator,
    {
        trace_generator.write(writer, self)
    }

    pub fn start(&mut self) -> super::Result<()> {
        self.entries.clear();
        self.start_time = time::SystemTime::now();

        #[cfg(feature = "gpu_queries")]
        if let Some(query_object) = &self.query_object {
            query_object.begin_query(QueryTarget::TimeElapsed)?;
        } else {
            self.query_object = Some(QueryObject::new());
            let query_object = self.query_object.as_ref().unwrap();
            query_object.begin_query(QueryTarget::TimeElapsed)?;
        }

        Ok(())
    }

    pub fn end(&mut self) -> super::Result<()> {
        #[cfg(feature = "gpu_queries")]
        if let Some(query_object) = &self.query_object {
            query_object.end_query(QueryTarget::TimeElapsed)?;
        }
        Ok(())
    }

    pub fn new_entry(&mut self, entry: TraceEntry) {
        let rc = Rc::new(entry);
        self.entries.push(rc);
    }
}

impl TraceEntry {
    pub fn new(function: &str) -> Self {
        Self {
            function: String::from(function),
            params: None,
            time_start: None,
            time_end: None,
        }
    }

    pub fn with_param(&mut self, name: &str, value: TraceEntryParamValue) -> &Self {
        if let None = self.params {
            self.params = Some(Vec::new());
        }
        self.params.as_mut().unwrap().push(TraceParam(
            String::from(name),
            TraceEntryParamValue::from(value),
        ));

        self
    }

    pub fn with_start_time(self) -> Self {
        return TraceEntry {
            time_start: Some(time::SystemTime::now()),
            ..self
        };
    }

    pub fn with_end_time(self) -> Self {
        return TraceEntry {
            time_end: Some(time::SystemTime::now()),
            ..self
        };
    }
}
