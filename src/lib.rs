#[path = "ppm.rs"]
mod ppm;

extern crate libc;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(ppm::max(2.into(),3.into()), 3);

        let img = ppm::Image{
            width: 8,
            height: 8,
            pixels: Vec::new()
        };
        ppm::write_ppm(img);
    }
}
