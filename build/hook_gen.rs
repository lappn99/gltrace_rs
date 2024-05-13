use gl_generator::{Generator, Registry};
use std::io;

pub struct HookGenerator;

impl Generator for HookGenerator {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write_header(dest)?;
        write_hookfuncs(registry, dest)?;
        write_match(registry, dest)
    }
}

fn write_header<W>(dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    write!(
        dest,
        r#"     pub mod __gl_imports {{
                pub use std::os::raw;
        }}"#
    )
}



fn write_hookfuncs<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    for cmd in &registry.cmds {
        writeln!(
            dest,
            r#"
                    
                    #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code,unused_mut)]
                    pub unsafe extern "C" fn gl{name}({params}) -> {return_type}{{
                        
                        let hook = crate::GLHooker::get_hook("gl{name}").unwrap();
                        let trace = hook.get_userdata_mut::<crate::Trace>().unwrap();
                        let mut trace_entry = crate::trace::TraceEntry::new("{name}");
                        with_params!(&mut trace_entry;{entry_params});
                        
                        if let Ok(addr) = hook.get_target_function() {{
                            let gl_func = core::mem::transmute::<*mut core::ffi::c_void, extern "C" fn({type_signature}) -> {return_type}>(addr);
                            let trace_entry = trace_entry.with_start_time();
                            let result = gl_func({arg_values});
                            let trace_entry = trace_entry.with_end_time();
                            trace.entries.push(trace_entry);
                            result
                        }} else {{
                            panic!();
                        }}
                        
                    }}
                "#,
            name = cmd.proto.ident,
            return_type = cmd.proto.ty,
            params = cmd
                .params
                .iter()
                .map(|binding| { format!("{}: {}", binding.ident, binding.ty) })
                .collect::<Vec<String>>()
                .join(", "),
            arg_values = cmd
                .params
                .iter()
                .map(|binding| { format!("{}", binding.ident) })
                .collect::<Vec<String>>()
                .join(", "),
            type_signature = cmd
                .params
                .iter()
                .map(|binding| { format!("{}", binding.ty) })
                .collect::<Vec<String>>()
                .join(", "),
            entry_params = cmd
                .params
                .iter()
                .map(|binding| { format!(" param \"{}\"; crate::trace::enums::TraceEntryParamValue::from({}); {}", binding.ident, binding.ident,binding.ty) })
                .collect::<Vec<String>>()
                .join(", "),
        )?
    }
    Ok(())
}

fn write_match<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(dest,"pub fn get_hook(identifier: &str) -> Result<*mut core::ffi::c_void, Box<dyn std::error::Error>> {{")?;
    writeln!(dest, "match identifier {{")?;
    for cmd in &registry.cmds {
        writeln!(
            dest,
            r#"
                    "gl{name}" => Ok(gl{name} as *mut core::ffi::c_void),
                "#,
            name = cmd.proto.ident
        )?
    }
    writeln!(
        dest,
        r#"
                _ => Err(crate::errors::GLTraceError::HookNotFound(String::from(identifier)).into())
                
            "#
    )?;
    writeln!(dest, "}}")?;
    writeln!(dest, "}}")?;
    Ok(())
}
