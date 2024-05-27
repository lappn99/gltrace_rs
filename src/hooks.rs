use crate::types::types;
use crate::trace::{self, enums::*};
use std::rc::Rc;
use crate::trace::resource::{ ResourceTransaction, TransactionType};
use std::ffi::CStr;

macro_rules! with_params {
    (&mut $entry: tt; param $name: literal; $value: expr; $param_type:ty ) => {
        crate::trace::TraceEntry::with_param(&mut $entry, $name, $value);


    };
    (&mut $entry: tt; param $name: literal; $value: expr; $param_type:ty, $(param $next_name: literal; $next_value: expr; $next_param_type:ty),+) => {
        with_params!(&mut $entry; param $name; $value; $param_type);
        with_params!(&mut $entry; $(param $next_name; $next_value; $next_param_type),+ )
    };
    (&mut $entry:tt;) => {
        ()
    };
}

#[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code,unused_mut)]
pub unsafe extern "C" fn glCreateShader(shader_type: types::GLenum) -> types::GLuint {
    let hook = crate::GLHooker::get_hook("glCreateShader").unwrap();
    let trace = hook.get_userdata_mut::<crate::Trace>().unwrap();
    let mut trace_entry = crate::trace::TraceEntry::new("glCreateShader");

    
    trace_entry.with_param("shader_type", TraceEntryParamValue::UInt(shader_type));

    let trace_entry = trace_entry.with_start_time();
    let result = gl::funcs::CreateShader(shader_type);
    let trace_entry = trace_entry.with_end_time();

    let trace_entry = Rc::new(trace_entry);

    trace.resource_transactions.push(ResourceTransaction {
        resource: result,
        transaction_type: TransactionType::CreateShader,
        entry: Rc::clone(&trace_entry)
    });

    trace.entries.push(Rc::clone(&trace_entry));

    result
}

#[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code,unused_mut)]
pub unsafe extern "C" fn glShaderSource(shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint)
{

   
    let hook = crate::GLHooker::get_hook("glShaderSource").unwrap();
    let trace = hook.get_userdata_mut::<crate::Trace>().unwrap();
    let mut trace_entry = crate::trace::TraceEntry::new("glShaderSource");

    trace_entry.with_param("shader", TraceEntryParamValue::UInt(shader));
    trace_entry.with_param("count", TraceEntryParamValue::Int(count));
    trace_entry.with_param("string", TraceEntryParamValue::ConstConstBytePtr(string));
    trace_entry.with_param("length",TraceEntryParamValue::ConstIntPtr(length));


    let trace_entry = trace_entry.with_start_time();
    gl::funcs::ShaderSource(shader, count, string, length);
    let trace_entry = trace_entry.with_end_time();
    let trace_entry = Rc::new(trace_entry);

    let mut shader_source = String::new();
    for i in [0..count] {
       if let Some(string) = string.as_ref() {
            let string = CStr::from_ptr(*string);
            if let Ok(string) = string.to_str() {
                shader_source.push_str(string);
            }

       }
    }

    trace.resource_transactions.push(ResourceTransaction {
        resource: shader,
        entry: Rc::clone(&trace_entry),
        transaction_type: TransactionType::UpdateShaderSource(shader_source)
    });

    trace.entries.push(Rc::clone(&trace_entry));
}

include!(concat!(env!("OUT_DIR"), "/hooks.rs"));
