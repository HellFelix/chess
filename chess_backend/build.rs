extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=c_lib/*");

    cc::Build::new()
        .file("c_lib/bitboard.c")
        .compile("bitboard");
    cc::Build::new().file("c_lib/movegen.c").compile("movegen");
    cc::Build::new()
        .file("c_lib/targets/king.c")
        .compile("king");
    cc::Build::new()
        .file("c_lib/targets/knight.c")
        .compile("knight");
    cc::Build::new()
        .file("c_lib/targets/pawn.c")
        .compile("pawn");
    cc::Build::new()
        .file("c_lib/targets/sliders.c")
        .compile("sliders");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
