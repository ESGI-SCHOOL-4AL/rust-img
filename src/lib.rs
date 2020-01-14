#![allow(dead_code)]
mod ppm;
mod ppma_io;

use crate::ppm::Pixel;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn invert(source: *const c_char, dest: *const c_char) {
    read_filter_write(source, dest, Pixel::invert)
}

#[no_mangle]
pub extern "C" fn grayscale(source: *const c_char, dest: *const c_char) {
    read_filter_write(source, dest, Pixel::grayscale)
}

fn read_filter_write<Filter>(source: *const c_char, dest: *const c_char, filter: Filter)
where
    Filter: Fn(&Pixel) -> Pixel,
    Filter: Send + Sync,
{
    let source = unsafe {
        CStr::from_ptr(source)
            .to_str()
            .expect("invalid pointer to source c-string")
    };
    let dest = unsafe {
        CStr::from_ptr(dest)
            .to_str()
            .expect("invalid pointer to dest c-string")
    };
    ppm::write(&ppm::read(source).filter(filter), dest)
}

#[cfg(test)]
mod integration_extern_tests {
    use super::*;
    use crate::ppm::{Image, Pixel};
    use rand::Rng;
    use std::ffi::CString;
    use std::fs;

    fn random_image(height: i32, width: i32) -> Image {
        fn random_pixels(n: i32) -> Vec<Pixel> {
            let mut rng = rand::thread_rng();
            (0..n)
                .map(|_| {
                    Pixel::new(
                        rng.gen_range(0, 255),
                        rng.gen_range(0, 255),
                        rng.gen_range(0, 255),
                    )
                })
                .collect()
        }
        Image::new(height, width, random_pixels(height * width))
    }

    fn get_random_dimensions() -> (i32, i32) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(100, 200), rng.gen_range(100, 200))
    }

    #[test]
    fn integration_test_extern_interface() {
        let original_path = "_extern_original.ppm";
        let inverted_path = "_extern_inverted.ppm";
        let doubly_inverted_path = "_extern_doubly_inverted.ppm";
        let (height, width) = get_random_dimensions();
        let original = random_image(height, width);
        ppm::write(&original, original_path);
        invert(
            CString::new(original_path).unwrap().as_ptr(),
            CString::new(inverted_path).unwrap().as_ptr(),
        );
        invert(
            CString::new(inverted_path).unwrap().as_ptr(),
            CString::new(doubly_inverted_path).unwrap().as_ptr(),
        );
        let doubly_inverted = ppm::read(doubly_inverted_path);
        assert_eq!(original, doubly_inverted);
        fs::remove_file(original_path).unwrap();
        fs::remove_file(inverted_path).unwrap();
        fs::remove_file(doubly_inverted_path).unwrap();
    }
}
