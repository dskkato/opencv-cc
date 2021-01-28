use std::env;
use std::fs::copy;
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
    let include_paths = "windows/opencv/build/include";
    let link_paths = concat!(env!("CARGO_PKG_NAME"), "/windows/opencv/build/x64/vc15/lib");
    let link_libs = "opencv_world451";

    println!("cargo:rustc-link-search={}", link_paths);
    println!("cargo:rustc-link-lib={}", link_libs);

    let src = format!("{}/{}.dll", "windows/opencv/build/x64/vc15/bin/", link_libs);
    let dst = format!("../{}.dll", link_libs);
    copy(src, dst).expect("failed to copy dll");

    cc::Build::new()
        .cpp(true)
        .file("src/cpp/wrapper.cc")
        .include("src/cpp")
        .include(include_paths)
        .compile("wrapper");
}
