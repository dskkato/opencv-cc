use image::{ImageBuffer, Rgb};
use std::env;
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn write_as_gray(fname:&str, imgbuf: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
  let fname = CString::new(fname).unwrap();
    unsafe {
        write_from_opencv(
          fname.as_ptr(),
            imgbuf.height() as i32,
            imgbuf.width() as i32,
            imgbuf.as_ptr() as *mut u8,
        );
    }
}
