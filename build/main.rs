use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, Profile, Registry};

use crate::hook_gen::HookGenerator;

mod hook_gen;

fn main() {
    println!("Running build script!");
    let dest = env::var("OUT_DIR").unwrap();
    let ver_major: u8  = env::var("GLTRACE_OPENGL_VER_MAJOR").unwrap_or(String::from("4")).parse::<u8>().unwrap();
    let ver_minor: u8  = env::var("GLTRACE_OPENGL_VER_MINOR").unwrap_or(String::from("5")).parse::<u8>().unwrap();
    let profile: Profile = match env::var("GLTRACE_OPENGL_PROFILE").unwrap_or(String::from( "CORE")).as_str() {
        "CORE" => Profile::Core,
        "COMPAT" => Profile::Compatibility,
        _ => Profile::Core
    };

    println!("Generating hooks for OpenGL version {}.{}, {:?} profile", ver_major, ver_minor, profile);

    let mut file = File::create(&Path::new(&dest).join("hooks.rs")).unwrap();
    Registry::new(Api::Gl, (ver_major, ver_minor), profile, Fallbacks::None, [])
        .write_bindings(HookGenerator, &mut file)
        .unwrap();
}
