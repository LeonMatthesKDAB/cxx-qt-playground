#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("src/myobject.h");

        type MyObject;
        fn myTest();
    }
}
