use std::ffi::CString;

mod cpp {
    use std::env;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn write_as_gray(fname: &str, height: i32, width: i32, buf: *const u8) {
    let fname = CString::new(fname).unwrap();
    unsafe {
        cpp::write_as_gray(fname.as_ptr(), height, width, buf as *mut u8);
    }
}

pub fn write_as_rgb(fname: &str, height: i32, width: i32, buf: *const u8) {
    let fname = CString::new(fname).unwrap();
    unsafe {
        cpp::write_as_rgb(fname.as_ptr(), height, width, buf as *mut u8);
    }
}
