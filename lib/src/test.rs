extern crate test;

use super::*;
use rand::Rng;
use test::Bencher;

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

#[bench]
fn bench_invert(b: &mut Bencher) {
    let mut invert_img = ppm::read_ppm("invert_src.ppm");
    b.iter(|| ppm::invert(&mut invert_img, "invert.ppm"));
}

#[bench]
fn bench_grayscale(b: &mut Bencher) {
    let mut grayscale_img = ppm::read_ppm("invert_src.ppm");
    b.iter(|| ppm::grayscale(&mut grayscale_img, "grayscale.ppm"));
}
