#[path = "ppm.rs"]
mod ppm;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(ppm::max(2,3), 3);
    }
}
