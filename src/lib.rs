#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        #[cxx_name="MyObjectRust"]
        pub type MyObject;

        fn get_prop(self: &MyObject) -> i32;
        fn set_prop(cpp: Pin<&mut MyObjectQt>, value: i32);
        fn create_rs() -> Box<MyObject>;

        // Rust::MyObject::add(this);
        fn add(cpp: Pin<&mut MyObjectQt>);
        // this would be better but unfortunately isn't available in CXX:
        // fn add(self: Pin<&mut MyObjectQt>);
    }

    unsafe extern "C++" {
        include!("myobject.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        fn rust(self: &MyObjectQt) -> &MyObject;
        #[cxx_name = "propChanged"]
        fn prop_changed(self: Pin<&mut MyObjectQt>);
        //fn my_thing(self: &MyObjectCpp) -> bool;
    }

    extern "C++" {
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }
}

/* Better scoping:*/
pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;
    use std::pin::Pin;

    pub struct MyObject {
        pub prop: i32,
    }
    
    impl MyObject {
        pub fn get_prop(self: &MyObject) -> i32 {
            self.prop
        }
    }

    impl MyObjectQt {
        pub fn set_prop(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().prop = value;
            }
            self.as_mut().prop_changed();
        }
    
        pub fn get_prop(self: &Self) -> i32 {
            self.rust().prop
        }
    
        pub fn add(mut self: Pin<&mut Self>) {
            let prop = self.as_ref().get_prop();
            self.as_mut().set_prop(prop + 1);
        }
    }

    #[cxx::bridge(namespace = "myobject::internals")]
    mod ffi {
        extern "Rust" {
            fn add(cpp: Pin<&mut MyObjectQt>);
        }

        unsafe extern "C++" {
            #[namespace = ""]
            #[cxx_name = "MyObject"]
            type MyObjectQt = super::super::ffi::MyObjectQt;
        }
    }

    pub fn create_rs() -> Box<MyObject> {
        Box::new(MyObject { prop: 42 })
    }
    
    pub fn set_prop(cpp: Pin<&mut ffi::MyObjectQt>, value: i32) {
        cpp.set_prop(value);
    }
    
    // this can work as our wrapper function, if necessary
    pub fn add(cpp: Pin<&mut ffi::MyObjectQt>) {
        cpp.add();
    }
    
}

/*
impl ffi::MyObjectQt {
    pub fn set_prop(mut self: Pin<&mut Self>, value: i32) {
        unsafe {
            self.as_mut().rust_mut().prop = value;
        }
        self.as_mut().prop_changed();
    }

    pub fn get_prop(self: &Self) -> i32 {
        self.rust().prop
    }

    pub fn add(mut self: Pin<&mut Self>) {
        let prop = self.as_ref().get_prop();
        self.as_mut().set_prop(prop + 1);
    }

    // fn private_stuff(self: Pin<&mut Self>) -> &MyCustomRustType {
    //     unsafe { self.get_mut(). }
    // }
}*/

//pub fn my_invokable(...) -> UniquePtr<QColor>

