use crate::ppma_io;
use rayon::prelude::*;
use std::ffi::CString;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pixel {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Image {
    width: i32,
    height: i32,
    pixels: Vec<Pixel>,
}

pub fn read(path: &str) -> Image {
    let mut height = 0;
    let mut width = 0;
    let mut rgb_max = 0;
    let mut r = std::ptr::null_mut();
    let mut g = std::ptr::null_mut();
    let mut b = std::ptr::null_mut();
    let cstring = CString::new(path.to_owned()).expect("CString::new failed");
    unsafe {
        ppma_io::ppma_read(
            cstring.as_ptr(),
            &mut width,
            &mut height,
            &mut rgb_max,
            &mut r,
            &mut g,
            &mut b,
        );
    }
    Image::from_raw_parts(height, width, (r, g, b))
}

/// The String and str types both implements AsRef<Path>
pub fn write<T: AsRef<Path>>(img: &Image, dest: T) {
    let path = CString::new(dest.as_ref().to_str().unwrap()).expect("CString::new failed");
    let (mut r, mut g, mut b) = img.rgb_vectors();
    unsafe {
        ppma_io::ppma_write(
            path.as_ptr(),
            img.width,
            img.height,
            r.as_mut_ptr(),
            g.as_mut_ptr(),
            b.as_mut_ptr(),
        );
    }
}

impl Pixel {
    pub fn new(red: i32, green: i32, blue: i32) -> Self {
        Pixel { red, green, blue }
    }

    pub fn grayscale(&self) -> Self {
        let gray =
            (self.red as f32 * 0.3 + self.green as f32 * 0.58 + self.blue as f32 * 0.11) as i32;
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

type RgbPointers = (*const i32, *const i32, *const i32);

impl Image {
    pub fn new(height: i32, width: i32, pixels: Vec<Pixel>) -> Self {
        Image {
            height,
            width,
            pixels,
        }
    }

    /// Will apply the given pixel filter to any given Image struct
    ///
    /// Individual pixel filters can be defined as a pure projections
    /// from one Pixel space to another Pixel space
    /// such that for one Pixel x there exists one Pixel y
    pub fn filter<Filter>(&self, filter: Filter) -> Self
    where
        Filter: Fn(&Pixel) -> Pixel,
        Filter: Send + Sync,
    {
        Image::new(
            self.width,
            self.height,
            self.pixels.par_iter().map(filter).collect(),
        )
    }

    fn rgb_vectors(&self) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
        self.pixels.iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut r, mut g, mut b), pixel| {
                r.push(pixel.red);
                g.push(pixel.green);
                b.push(pixel.blue);
                (r, g, b)
            },
        )
    }

    fn from_raw_parts(height: i32, width: i32, rgb: RgbPointers) -> Self {
        unsafe fn pixel_at(offset: isize, rgb: RgbPointers) -> Pixel {
            Pixel::new(
                *rgb.0.offset(offset),
                *rgb.1.offset(offset),
                *rgb.2.offset(offset),
            )
        }
        Image::new(
            height,
            width,
            (0..(height * width) as isize)
                .map(|i| unsafe { pixel_at(i, rgb) })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
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
    fn should_read_the_same_image_as_was_written() {
        let (height, width) = get_random_dimensions();
        let original = random_image(height, width);
        let path = "img.ppm";
        write(&original, path);
        let read_img = read(path);
        assert_eq!(original, read_img);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_inverted_image_should_invert_to_original() {
        let (height, width) = get_random_dimensions();
        let original = random_image(height, width);
        let inverted = original.filter(Pixel::invert);
        let doubly_inverted = inverted.filter(Pixel::invert);
        assert_ne!(original, inverted);
        assert_eq!(original, doubly_inverted);
    }

    #[test]
    fn integration_test() {
        let inverted_path = "_intern_inverted.ppm";
        let grayscaled_path = "_intern_grayscaled.ppm";
        let (height, width) = get_random_dimensions();
        let original = random_image(height, width);
        let inverted = original.filter(Pixel::invert);
        let grayscaled = original.filter(Pixel::grayscale);
        write(&inverted, inverted_path);
        write(&grayscaled, grayscaled_path);
        let read_inverted = read(inverted_path);
        let read_grayscaled = read(grayscaled_path);
        assert_eq!(inverted, read_inverted);
        assert_eq!(grayscaled, read_grayscaled);
        let doubly_inverted = read_inverted.filter(Pixel::invert);
        assert_eq!(original, doubly_inverted);
        fs::remove_file(inverted_path).unwrap();
        fs::remove_file(grayscaled_path).unwrap();
    }

    #[test]
    fn test_from_raw_parts_regression() {
        fn old_from_raw_parts(height: i32, width: i32, rgb: RgbPointers) -> Image {
            let mut pixels = Vec::new();
            let (r, g, b) = rgb;
            for y in 0..height {
                let offset = (y * width) as isize;
                for x in 0..width {
                    unsafe {
                        pixels.push(Pixel {
                            red: *r.offset(offset + x as isize),
                            green: *g.offset(offset + x as isize),
                            blue: *b.offset(offset + x as isize),
                        });
                    }
                }
            }
            Image {
                height,
                width,
                pixels,
            }
        }
        let (height, width) = get_random_dimensions();
        let img1 = random_image(height, width);
        let (mut r, mut g, mut b) = img1.rgb_vectors();
        let img2 = Image::from_raw_parts(
            height,
            width,
            (r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()),
        );
        let img3 = old_from_raw_parts(
            height,
            width,
            (r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()),
        );
        assert_eq!(img1, img2);
        assert_eq!(img2, img3);
    }
}
