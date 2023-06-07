#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn test() -> ();
    }
}

use cxxqt_ffi::*;
mod cxxqt_ffi {
    pub fn test((x, y): (i32, i32)) {}
}

