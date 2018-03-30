extern crate bindgen;
extern crate gcc;

fn main() {
    gen_lib();
    gen_bindings();
}

const NUKE_C: &str = "c/nuke.c";
const LIBNUKE: &str = "libnuke.a";

#[cfg(not(debug_assertions))]
fn gen_lib() {
    gcc::Config::new()
            .file(NUKE_C)
            .define("NK_ASSERT", None)
            .define("NDEBUG", None)
            .compile(LIBNUKE);
}

#[cfg(all(debug_assertions, feature="no_libc"))]
fn gen_lib() {
    gcc::Config::new()
            .file(NUKE_C)
            .define("NK_ASSERT", None)
            .compile(LIBNUKE);
}
#[cfg(all(debug_assertions, not(feature="no_libc")))]
fn gen_lib() {
    gcc::Config::new()
            .file(NUKE_C)
            .compile(LIBNUKE);
}

#[cfg(not(feature="gen_bindings"))]
fn gen_bindings() {}

#[cfg(feature="gen_bindings")]
fn gen_bindings() {
    use std::env;
    use std::path::PathBuf;
    let bindings = bindgen::Builder::default()
        .header("c/nuke.h")
        .generate_comments(true)
        .derive_debug(true)
        .use_core()
        .ctypes_prefix("libc")
        .no_unstable_rust()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}