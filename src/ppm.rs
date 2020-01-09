extern crate libc;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use libc::{int32_t};

#[link(name = "ppma_io")]
extern "C" {
    fn ch_cap(ch: char) -> int32_t;
    fn i4_max(i1: int32_t, i2: int32_t) -> int32_t;
    fn ppma_read(input_name: *const libc::wchar_t, xsize: *mut int32_t, ysize: *mut int32_t, rgb_max: *mut int32_t);
    fn ppma_write(file_out_name: *const c_char, xsize: int32_t, ysize: int32_t, r: *mut int32_t, g: *mut int32_t, b: *mut int32_t);
}

pub struct Pixel {
    red: i8,
    green: i8,
    blue: i8,
}

pub struct Image {
    // TODO img path
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Pixel>,
}

pub fn max(i1: i32, i2: i32) -> i32 {
    return unsafe { i4_max(i1, i2) };
}

pub fn read_ppm() -> Image {
    let mut img = Image {
        width: 0,
        height: 0,
        pixels: Vec::new()
    };

    return img;
}

pub fn write_ppm(img: Image) {
    let img_name = CString::new("img.ppm").expect("CString::new failed");

    unsafe {
        let mut r: [c_int; 64] = [0; 64];
        let mut g: [c_int; 64] = [0; 64];
        let mut b: [c_int; 64] = [0; 64];
        ppma_write(img_name.as_ptr(), img.width, img.height, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr());
    }
}