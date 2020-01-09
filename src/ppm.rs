extern crate libc;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[link(name = "ppma_io")]
extern "C" {
    fn i4_max(i1: i32, i2: i32) -> i32;
    fn ppma_read(input_name: *const c_char, xsize: *mut i32, ysize: *mut i32, rgb_max: *mut i32, r: *mut *mut i32, g: *mut *mut i32, b: *mut *mut i32);
    fn ppma_write(file_out_name: *const c_char, xsize: i32, ysize: i32, r: *mut i32, g: *mut i32, b: *mut i32);
}

pub struct Pixel {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

pub struct Image {
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Pixel>,
}

pub fn max(i1: i32, i2: i32) -> i32 {
    return unsafe { i4_max(i1, i2) };
}

pub fn read_ppm(path: &str) -> Image {
    unsafe {
        let mut img = Image {
            path: String::from(path),
            width: 0,
            height: 0,
            pixels: Vec::new()
        };

        let mut rgb_max: i32 = 0;
        let mut r: *mut i32 = std::ptr::null_mut();
        let mut g: *mut i32 = std::ptr::null_mut();
        let mut b: *mut i32 = std::ptr::null_mut();

        ppma_read(CString::new(img.path.clone()).expect("CString::new failed").as_ptr(), &mut img.width, &mut img.height, &mut rgb_max, &mut r, &mut g, &mut b);
        
        for y in 0..img.height {
            let offset = (y * img.width) as isize;
            for x in 0..img.width {
                img.pixels.push(Pixel{
                    red: *r.offset(offset + x as isize),
                    green: *g.offset(offset + x as isize),
                    blue: *b.offset(offset + x as isize),
                });
            }
        }

        return img;
    }
}

pub fn write_ppm(img: Image) {
    let img_name = CString::new(img.path).expect("CString::new failed");

    unsafe {
        let size = (img.width * img.height) as usize;
        let mut r = Vec::with_capacity(size);
        let mut g = Vec::with_capacity(size);
        let mut b = Vec::with_capacity(size);

        for p in &img.pixels {
            r.push(p.red);
            g.push(p.green);
            b.push(p.blue);
        }
        ppma_write(img_name.as_ptr(), img.width, img.height, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr());
    }
}