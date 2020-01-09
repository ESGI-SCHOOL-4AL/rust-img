#[path = "ppm.rs"]
mod ppm;

extern crate libc;
extern crate rand;

use rand::Rng;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(ppm::max(2.into(),3.into()), 3);

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
    }
}
