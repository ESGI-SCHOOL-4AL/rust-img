// #[link(name = "ppma_io")]
// extern {
//     fn i4_max(i1: i8, i2: i8) -> i8;
// }

#[link(name = "ppma_io")]
extern "C" {
    fn i4_max(i1: i8, i2: i8) -> i8;
}

pub struct Pixel {
    red: i8,
    green: i8,
    blue: i8,
}

pub struct Image {
    pixels: Vec<Vec<Pixel>>
}

pub fn max(i1: i8, i2: i8) -> i8 {
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