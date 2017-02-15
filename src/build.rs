extern crate ructe;

use ructe::compile_templates;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("templates");
    compile_templates(&in_dir, &out_dir).expect("Can't compile templates");
}
