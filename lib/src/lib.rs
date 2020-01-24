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
mod tests {

    use super::*;
    use rand::Rng;

    #[test]
    fn it_works() {
        let size = 64;
        let mut img = ppm::Image{
            path: String::from("img.ppm"),
            width: size,
            height: size,
            pixels: Vec::new()
        };

        let mut rng = rand::thread_rng();
        for _ in 0..size.pow(2) {
            img.pixels.push(ppm::Pixel{
                red: rng.gen_range(0,255),
                green: rng.gen_range(0,255),
                blue: rng.gen_range(0,255),
            });
        }

        ppm::write_ppm(img);

        let read_img = ppm::read_ppm("img.ppm");
        assert_eq!(read_img.width, size);
        assert_eq!(read_img.height, size);

        let cpy_img = ppm::Image{
            path: String::from("img_cpy.ppm"),
            width: read_img.width,
            height: read_img.height,
            pixels: read_img.pixels
        };

        ppm::write_ppm(cpy_img);

        let mut invert_img = ppm::read_ppm("invert_src.ppm");
        ppm::invert(&mut invert_img, "invert.ppm");

        let mut grayscale_img = ppm::read_ppm("invert_src.ppm");
        ppm::grayscale(&mut grayscale_img, "grayscale.ppm");
    }
}
