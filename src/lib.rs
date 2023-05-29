#[cxx::bridge]
mod ffi {
    extern "Rust" {
        #[rust_name="MyStruct"]
        type MyStructRust;
    }
}

struct MyStruct {
    my_thing: i32
}