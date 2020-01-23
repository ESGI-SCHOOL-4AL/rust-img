extern crate libc;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::thread;
use std::sync::{Mutex, Arc};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator}; // 1.0.3
use std::{time::Duration};

static THREAD: i8 = 2;
static GREYSCALE_RED: f32 = 0.3;
static GREYSCALE_GREEN: f32 = 0.58;
static GREYSCALE_BLUE: f32 = 0.11;


#[link(name = "ppma_io")]
extern "C" {
    fn i4_max(i1: i32, i2: i32) -> i32;
    fn ppma_read(input_name: *const c_char, xsize: *mut i32, ysize: *mut i32, rgb_max: *mut i32, r: *mut *mut i32, g: *mut *mut i32, b: *mut *mut i32);
    fn ppma_write(file_out_name: *const c_char, xsize: i32, ysize: i32, r: *mut i32, g: *mut i32, b: *mut i32);
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pixel {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}



#[derive(Clone)]
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

pub fn invert(src: &mut Image, dst: &str) {
    let image_path: String = String::from(dst);
    let image_width: i32 = src.width;
    let image_height: i32 = src.height;

<<<<<<< Updated upstream
    let arc = Arc::new(Mutex::new(image_pixels));

    for pixels_for_thread in splited_pixels_for_threads {
        let pixels = Arc::clone(&arc);

        let thread = thread::spawn(move || {
            let mut pixels_locked = pixels.lock().unwrap();
                for p in pixels_for_thread {
                    pixels_locked.push(Pixel{
                        red: 255 - p.red,
                        green: 255 - p.green,
                        blue: 255 - p.blue,
            
                    })
                }
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
=======
    src.pixels.par_iter_mut().for_each(|pixel| {
            pixel.red = 255 - pixel.red;
            pixel.green = 255 - pixel.green;
            pixel.blue = 255 - pixel.blue;
        
    });
>>>>>>> Stashed changes

    write_ppm(Image {
        path: image_path,
        width: image_width,
        height: image_height,
        pixels: src.pixels.to_owned(),
    });
}

pub fn grayscale(src: Image, dst: &str) {
    let vec_size_foreach_thread: u32 = src.pixels.len() as u32 / THREAD as u32;
    let splited_pixels_for_threads: Vec<Vec<Pixel>> = split_vec_by_thread(src.pixels, vec_size_foreach_thread);
    let mut threads = Vec::new();

    let image_path: String = String::from(dst);
    let image_width: i32 = src.width;
    let image_height: i32 = src.height;
    let image_pixels = Vec::new();
    
    let arc = Arc::new(Mutex::new(image_pixels));

    for pixels_for_thread in splited_pixels_for_threads {
        let pixels = Arc::clone(&arc);

        let thread = thread::spawn(move || {
            let mut image_locked = pixels.lock().unwrap();
            for p in pixels_for_thread {
                let gray = (p.red as f32 * GREYSCALE_RED + p.green as f32 * GREYSCALE_GREEN + p.blue as f32 * GREYSCALE_BLUE) as i32;
                image_locked.push(Pixel{
                    red: gray,
                    green: gray,
                    blue: gray,
                })
            }             
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    write_ppm(Image {
        path: image_path,
        width: image_width,
        height: image_height,
        pixels: arc.lock().unwrap().to_vec(),
    });
}

fn split_vec_by_thread(all_image_pixels: Vec<Pixel>, array_size_for_thread: u32) -> Vec<Vec<Pixel>> {
    let mut splited_image_vector = Vec::new();
    let mut current_image: Vec<Pixel> = all_image_pixels;
    
    if current_image.len() < array_size_for_thread as usize {
        for pixel in current_image {
            splited_image_vector.push(vec!(pixel));
        }

        return splited_image_vector;
    }

    while current_image.len() >= array_size_for_thread as usize {
        let (splited_vector_result, splited_vector_remnant) = current_image.split_at(array_size_for_thread as usize);
        splited_image_vector.push(splited_vector_result.to_vec());
        current_image = splited_vector_remnant.to_vec();
    }

    if current_image.len() > 0 {
        splited_image_vector.push(current_image);
    }
    
    return splited_image_vector;
}

#[cfg(test)]
mod tests {
    use super::split_vec_by_thread;
    use super::Pixel;

    static pixel_exemple: Pixel = Pixel {
        red: 0,
        green: 0,
        blue: 0,
    };
    
    #[test]
    fn split_6_by_2_vec() {
        let exemple: Vec<Pixel> = vec!(pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple);
        let expected: Vec<Vec<Pixel>> = vec!(vec!(pixel_exemple, pixel_exemple, pixel_exemple), vec!(pixel_exemple, pixel_exemple, pixel_exemple));
        assert_eq!(split_vec_by_thread(exemple, 3), expected);

    }

    #[test]
    fn split_7_by_2_vec() {
        let exemple: Vec<Pixel> = vec!(pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple);
        let expected: Vec<Vec<Pixel>> = vec!(vec!(pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple), vec!(pixel_exemple, pixel_exemple, pixel_exemple));
        assert_eq!(split_vec_by_thread(exemple, 4), expected);

    }

    #[test]
    fn split_6_by_3_vec() {
        let exemple: Vec<Pixel> = vec!(pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple);
        let expected: Vec<Vec<Pixel>> = vec!(vec!(pixel_exemple, pixel_exemple), vec!(pixel_exemple, pixel_exemple), vec!(pixel_exemple, pixel_exemple));
        assert_eq!(split_vec_by_thread(exemple, 2), expected);

    }

    #[test]
    fn split_7_by_3_vec() {
        let exemple: Vec<Pixel> = vec!(pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple, pixel_exemple);
        let expected: Vec<Vec<Pixel>> = vec!(vec!(pixel_exemple, pixel_exemple, pixel_exemple), vec!(pixel_exemple, pixel_exemple, pixel_exemple), vec!(pixel_exemple));
        assert_eq!(split_vec_by_thread(exemple, 3), expected);

    }

    #[test]
    fn split_2_by_4_vec() {
        let exemple: Vec<Pixel> = vec!(pixel_exemple, pixel_exemple);
        let expected: Vec<Vec<Pixel>> = vec!(vec!(pixel_exemple), vec!(pixel_exemple));
        assert_eq!(split_vec_by_thread(exemple, 4), expected);

    }
}