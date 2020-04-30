use std::path::PathBuf;
use std::env;

const HEADER_NAME: &'static str = "fswatch/libfswatch/src/libfswatch/c/libfswatch.h";
const LIBRARY_PATH: &'static str = "fswatch/libfswatch/src/libfswatch/.libs/";

#[cfg(any(target_os="macos", target_os="freebsd"))]
const CPP_LIB: &'static str = "c++";
#[cfg(target_os="linux")]
const CPP_LIB: &'static str = "stdc++";
// no fallthrough case: to support more OSes, add their default C++ library here
// an error about CPP_LIB being undefined should lead right here, but the linker errors could be much more confusing

fn main() {
    for lib in &["fswatch", CPP_LIB] {
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
