fn main() {
    cxx_build::bridges(["src/lib.rs"])
        .include("/home/kdab/Documents/projects/3371-Rust-RnD/playground")
        .file("/home/kdab/Documents/projects/3371-Rust-RnD/playground/src/myobject.cpp")
        .compile("cxx-demo");
}
