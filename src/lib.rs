#[path = "ppm.rs"]
mod ppm;

extern crate libc;
use libc::{size_t, int32_t};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(ppm::max(2.into(),3.into()), 3);
    }
}
