use gl_generator::{Generator, Registry};
use std::io;

pub struct HookGenerator<'a> {
    pub blacklist: Vec<&'a str>
}

impl<'a> Generator for HookGenerator<'a> {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write_header(dest)?;
        write_funcptrs(registry, dest)?;
        write_hookfuncs(registry, dest,&self.blacklist)?;
        write_match(registry, dest)
    }
}

fn write_header<W>(dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(
        dest,
        r#"     pub mod __gl_imports {{
                pub use std::os::raw;
        }}"#
    )
}

fn write_funcptrs<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(
        dest,
        "pub mod gl {{\nuse super::__gl_imports;\nuse super::types;"
    )?;
    writeln!(dest, "#[allow(non_camel_case_types)]\npub struct funcs;")?;
    writeln!(dest, "\nimpl funcs{{")?;
    for cmd in &registry.cmds {
        write!(
            dest,
            r#"
            #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code,unused_mut)]
            pub unsafe extern "C" fn {name}({params}) -> {return_type}{{
                let hook = crate::GLHooker::get_hook("gl{name}").unwrap();
                if let Ok(addr) = hook.get_target_function() {{
                    let gl_func = core::mem::transmute::<*mut core::ffi::c_void, extern "C" fn({type_signature}) -> {return_type}>(addr);
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
                .join(", ")
        )?;
    }

    writeln!(dest, "}}")?;
    writeln!(
        dest,
        "#[allow(non_camel_case_types)]\npub mod enums{{\n use super::types;"
    )?;

    for enm in &registry.enums {
        let types_prefix = "types::";
        writeln!(dest,
            "#[allow(dead_code, non_upper_case_globals)] pub const {ident}: {types_prefix}{ty} = {value}{cast_suffix};",
            ident = enm.ident,
            types_prefix = if enm.ty == "&'static str" { "" } else { types_prefix },
            ty = enm.ty,
            value = enm.value,
            cast_suffix = match enm.cast {
                true => format!(" as {}{}", types_prefix, enm.ty),
                false => String::new(),
            },
        )?;
    }

    writeln!(dest, "}}")?;
    writeln!(dest, "}}")?;
    Ok(())
}

fn write_hookfuncs<'a,W>(registry: &Registry, dest: &mut W, blacklist: &[&'a str]) -> io::Result<()>
where
    W: io::Write,
{

    for cmd in &registry.cmds.iter().filter(|c| !blacklist.contains(&&c.proto.ident[0..])).collect::<Vec<&gl_generator::Cmd>>() {

        let params = cmd
            .params
            .iter()
            .map(|binding| { format!("{}: {}", binding.ident, binding.ty) })
            .collect::<Vec<String>>()
            .join(", ");
        let name = &cmd.proto.ident;
        let return_type = &cmd.proto.ty;
        writeln!(
            dest, 
            r#"
                #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code,unused_mut)]
                pub unsafe extern "C" fn gl{name}({params}) -> {return_type}{{
        "#)?;

        let entry_params = cmd
            .params
            .iter()
            .map(|binding| {
                format!(
                    " param \"{}\"; crate::trace::enums::TraceEntryParamValue::from({}); {}",
                    binding.ident, binding.ident, binding.ty
                )
            })
            .collect::<Vec<String>>()
            .join(", ");

        writeln!(
            dest,
            r#"         
                    let hook = crate::GLHooker::get_hook("gl{name}").unwrap();
                    let trace = hook.get_userdata_mut::<crate::Trace>().unwrap();
                    let mut trace_entry = crate::trace::TraceEntry::new("{name}");
                    with_params!(&mut trace_entry;{entry_params});
            "#
        )?;

        writeln!(
            dest,
            r#"
          
                    if let Ok(addr) = hook.get_target_function() {{
                        let gl_func = core::mem::transmute::<*mut core::ffi::c_void, extern "C" fn({type_signature}) -> {return_type}>(addr);
                        let trace_entry = trace_entry.with_start_time();
                        let result = gl_func({arg_values});
                        let trace_entry = trace_entry.with_end_time();
                        let trace_entry = Rc::new(trace_entry);
                        trace.entries.push(Rc::clone(&trace_entry));
                        result
                    }} else {{
                        panic!();
                    }}
                        
                }}
                "#,
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
