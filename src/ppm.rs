// #[link(name = "ppma_io")]
// extern {
//     fn i4_max(i1: i8, i2: i8) -> i8;
// }

extern crate libc;
use libc::{size_t, int32_t};

#[link(name = "ppma_io")]
extern "C" {
    fn ch_cap(ch: char) -> int32_t;
    fn i4_max(i1: int32_t, i2: int32_t) -> int32_t;
    fn ppma_read(input_name: *mut char, xsize: *mut int32_t, ysize: *mut int32_t, rgb_max: *mut int32_t);
    fn ppma_write(file_out_name: *mut char, xsize: int32_t, ysize: int32_t, r: *mut int32_t, g: *mut int32_t, b: *mut int32_t);
}

pub struct Pixel {
    red: i8,
    green: i8,
    blue: i8,
}

pub struct Image {
    pixels: Vec<Vec<Pixel>>
}

pub fn max(i1: i32, i2: i32) -> i32 {
    return unsafe { i4_max(i1, i2) };
}

pub fn read_ppm() -> Image {
    let mut img = Image { 
        pixels: Vec::new()
    };

    return img;
}

pub fn write_ppm(img: Image) {

}