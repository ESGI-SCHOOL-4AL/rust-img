extern crate test;

use super::*;
use rand::Rng;
use test::Bencher;

#[test]
fn it_works() {
    let size = 64;
    let mut img = ppm::Image{
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

    img.save("img.ppm");

    let read_img = ppm::read_ppm("img.ppm");
    assert_eq!(read_img.width, size);
    assert_eq!(read_img.height, size);

    let cpy_img = ppm::Image{
        width: read_img.width,
        height: read_img.height,
        pixels: read_img.pixels
    };

    cpy_img.save("img_cpy.ppm");

    let invert_img = ppm::read_ppm("invert_src.ppm");
    invert_img.filter(invert_img.invert_pixels()).save("invert.ppm");

    let grayscale_img = ppm::read_ppm("invert_src.ppm");
    invert_img.filter(grayscale_img.grayscale_pixels()).save("grayscale.ppm");
}

#[bench]
fn bench_invert(b: &mut Bencher) {
    let invert_img = ppm::read_ppm("invert_src.ppm");
    b.iter(|| invert_img.invert_pixels());
}

#[bench]
fn bench_grayscale(b: &mut Bencher) {
    let grayscale_img = ppm::read_ppm("invert_src.ppm");
    b.iter(|| grayscale_img.grayscale_pixels());
}
