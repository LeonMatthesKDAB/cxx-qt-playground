#[cxx::bridge]
mod ffi {
    extern "C++" {
        type QString;

        fn toBytes(&self);
    }
}

impl ffi::QString {
    fn to_str(&self, ) -> String {

    }
}