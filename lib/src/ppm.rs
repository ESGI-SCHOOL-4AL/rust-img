extern crate libc;

use std::ffi::CString;
use std::os::raw::{c_char};
use rayon::prelude::*;

static GREYSCALE_RED: f32 = 0.3;
static GREYSCALE_GREEN: f32 = 0.58;
static GREYSCALE_BLUE: f32 = 0.11;


#[link(name = "ppma_io")]
extern "C" {
    fn ppma_read(
        input_name: *const c_char, 
        xsize: *mut i32, 
        ysize: *mut i32, 
        rgb_max: *mut i32, 
        r: *mut *mut i32, 
        g: *mut *mut i32, 
        b: *mut *mut i32
    );
    fn ppma_write(
        file_out_name: *const c_char, 
        xsize: i32, 
        ysize: i32, 
        r: *mut i32, 
        g: *mut i32, 
        b: *mut i32
    );
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pixel {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl Pixel {
    pub fn grayscale(&self) -> Self {
        let gray = (self.red as f32 * GREYSCALE_RED + self.green as f32 * GREYSCALE_GREEN + self.blue as f32 * GREYSCALE_BLUE) as i32;
        Pixel {
            red: gray,
            green: gray,
            blue: gray,
        }
    }

    pub fn invert(&self) -> Self {
        Pixel {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue,
        }
    }
}


#[derive(Clone)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn save(&self, dst: &str) {
        let path = CString::new(dst).expect("CString::new failed");
    
        unsafe {
            let size = (self.width * self.height) as usize;
            let mut r = Vec::with_capacity(size);
            let mut g = Vec::with_capacity(size);
            let mut b = Vec::with_capacity(size);
    
            for p in &self.pixels {
                r.push(p.red);
                g.push(p.green);
                b.push(p.blue);
            }
            ppma_write(path.as_ptr(), self.width, self.height, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr());
        }
    }

    pub fn filter(&self, pixels: Vec<Pixel>) -> Image {
        return Image {
            width: self.width,
            height: self.height,
            pixels: pixels,
        };
    }

    pub fn invert_pixels(&self) -> Vec<Pixel> {
        return self.pixels.par_iter().map(Pixel::invert).collect();
    }

    pub fn grayscale_pixels(&self) -> Vec<Pixel> {
        return self.pixels.par_iter().map(Pixel::grayscale).collect();
    }
}

pub fn read_ppm(path: &str) -> Image {
    unsafe {
        let mut img = Image {
            width: 0,
            height: 0,
            pixels: Vec::new()
        };

        let mut rgb_max: i32 = 0;
        let mut r: *mut i32 = std::ptr::null_mut();
        let mut g: *mut i32 = std::ptr::null_mut();
        let mut b: *mut i32 = std::ptr::null_mut();

        ppma_read(CString::new(path).expect("CString::new failed").as_ptr(), &mut img.width, &mut img.height, &mut rgb_max, &mut r, &mut g, &mut b);
        
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
