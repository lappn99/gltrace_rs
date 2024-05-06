use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, Profile, Registry};

use crate::hook_gen::HookGenerator;

mod hook_gen;

fn main() {
    println!("Running build script!");
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("hooks.rs")).unwrap();
    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::None, [])
        .write_bindings(HookGenerator, &mut file)
        .unwrap();
}
