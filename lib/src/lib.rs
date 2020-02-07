#![feature(test)]

#[path = "ppm.rs"]
mod ppm;

extern crate libc;
extern crate rand;

use std::ffi::CStr;

fn cstr_to_string(s: *const libc::c_char) -> String {
    let buf = unsafe { CStr::from_ptr(s).to_bytes() };
    return String::from_utf8(buf.to_vec()).unwrap();
}

#[no_mangle]
pub extern "C" fn invert(file: *const libc::c_char, destination: *const libc::c_char) {
    let src = cstr_to_string(file);
    let dst = cstr_to_string(destination);

    let mut src_img = ppm::read_ppm(&src);
    ppm::invert(&mut src_img, &dst);
}

#[no_mangle]
pub extern "C" fn grayscale(file: *const libc::c_char, destination: *const libc::c_char) {
    let src = cstr_to_string(file);
    let dst = cstr_to_string(destination);

    let mut src_img = ppm::read_ppm(&src);
    ppm::grayscale(&mut src_img, &dst);
}

#[cfg(test)]
#[path = "test.rs"]
mod tests;
