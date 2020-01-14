use std::os::raw::c_char;

#[link(name = "ppma_io")]
extern "C" {
    pub fn ppma_read(
        input_name: *const c_char,
        xsize: *mut i32,
        ysize: *mut i32,
        rgb_max: *mut i32,
        r: *mut *mut i32,
        g: *mut *mut i32,
        b: *mut *mut i32,
    );

    pub fn ppma_write(
        file_out_name: *const c_char,
        xsize: i32,
        ysize: i32,
        r: *mut i32,
        g: *mut i32,
        b: *mut i32,
    );
}
