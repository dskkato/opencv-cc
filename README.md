# opencv-cc

This repository demonstrate how to use OpenCV lib from C++ inside a Rust project.

## Motivation

I know that there is already a whole OpenCV wrapper for Rust.

[opencv-rust](https://github.com/twistedfall/opencv-rust)

However, usingh that crate is usually painfull bacause of

* Little documentations in Rust
* Hard interoperability between C++ lib and Rust code

While using OpenCV in C++ might be easier than that over opencv-rust, building process becomes complex due to FFI construction.

This repository aims to demonstrate minimum setups to use OpenCV/C++ in a Rust project.

Please note that depending FFI means language supports (both C++ and Rust) are sacrificed.

## Build steps

1. Writing C++ wrapper which uses OpneCV funcionalities.
2. Generate Rust code with `bindgen`
3. Write `build.rs` to resolve required symbols.

### Environment variables

* `OPENCV_LINK_LIBS` Indicate one lib `opencv_worldXXX` where `XXX` is the library's version.
* `OPENCV_LINK_PATHS` Path to ths lib file location
* `OPENCV_INCLUDE_PATHS` Path to OpenCV's headers.

These name convention came from `opencv-rust` but only one path or lib can be set for each, so that what you can use is `opencv_world` configuration. In order to suplly multiple files/paths, see `opencv-rust` build.rs script.

## License

MIT