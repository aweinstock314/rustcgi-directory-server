use std::path::PathBuf;
use std::env;

const HEADER_NAME: &'static str = "fswatch/libfswatch/src/libfswatch/c/libfswatch.h";
const LIBRARY_PATH: &'static str = "fswatch/libfswatch/src/libfswatch/.libs/";

fn main() {
    let cpp_lib = match &*env::var("CARGO_CFG_TARGET_OS").unwrap() {
        "macos" | "freebsd" => "c++",
        "linux" => "stdc++",
        other => panic!("Haven't yet researched which C++ standard library to link with on TARGET_OS {}", other),
    };

    for lib in &["fswatch", cpp_lib] {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
    println!("cargo:rerun-if-changed={}", HEADER_NAME);

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/{}", dir, LIBRARY_PATH);

    let bindings = bindgen::Builder::default()
        .header(HEADER_NAME)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("fswatch_sys.rs"))
        .expect("Couldn't write bindings!");
}
