use gl_generator::{Binding, Generator, Registry};
use std::{fmt::format, io};

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
                    
                    #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
                    pub unsafe extern "C" fn gl{name}({params}) -> {return_type}{{
                        println!("Call {name} with {arg_names}", {arg_values});
                        if let Ok(addr) = crate::GLHooker::get_original_function() {{
                            let gl_func = core::mem::transmute::<*mut core::ffi::c_void, extern "C" fn({type_signature}) -> {return_type}>(addr);
                            crate::GLTrace::trace_call().unwrap();
                            gl_func({arg_values})
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
            arg_names = cmd
                .params
                .iter()
                .map(|binding| { format!("{}: {{:?}}", binding.ident) })
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
