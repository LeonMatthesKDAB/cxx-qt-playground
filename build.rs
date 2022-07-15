// build.rs

// fn main() {
//     cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        

//     println!("cargo:rerun-if-changed=src/main.rs");
//     println!("cargo:rerun-if-changed=src/demo.cc");
//     println!("cargo:rerun-if-changed=include/demo.h");
// }
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/lib.rs")
        .disable_qt()
        .build();
}
