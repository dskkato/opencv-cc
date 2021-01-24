use std::env;
use std::path::PathBuf;

fn main() {
    // `bindgen` part
    println!("cargo:rerun-if-changed=src/cpp/wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("src/cpp/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Get OpenCV path and target lib through env variables.
    let include_paths =
        env::var("OPENCV_INCLUDE_PATHS").expect("Please set `OPENCV_INCLUDE_PATHS` env var.");
    let link_paths =
        env::var("OPENCV_LINK_PATHS").expect("Please set `OPENCV_LINK_PATHS` env var.");
    let link_libs = env::var("OPENCV_LINK_LIBS").expect("Please set `OPENCV_LINK_LIBS` env var.");

    println!("cargo:rustc-link-search={}", link_paths);
    println!("cargo:rustc-link-lib={}", link_libs);

    cc::Build::new()
        .cpp(true)
        .file("src/cpp/wrapper.cc")
        .include("src/cpp")
        .include(include_paths)
        .compile("wrapper");
}
