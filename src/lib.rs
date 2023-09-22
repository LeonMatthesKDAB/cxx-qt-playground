use std::{ffi::*, pin::Pin};

#[cxx::bridge(namespace = "test::ffi")]
pub mod ffi {
    unsafe extern "C++" {
        include!("/home/kdab/Documents/projects/3371-Rust-RnD/playground/src/myobject.h");
        type SignalHandlerMyClassMySignal = super::SignalHandler<super::MyClassMySignalClosure>;

        #[cxx_name = "connect_MyClass_mySignal"]
        fn connect_my_class_my_signal(obj: &MyClass, signal_handler: SignalHandlerMyClassMySignal);

        #[cxx_name = "create_my_class"]
        fn MyClass() -> *mut MyClass;

        type MyClass;
    }

    extern "Rust" {
        unsafe fn drop_signal_handler_my_class_my_signal(handler: SignalHandlerMyClassMySignal);

        fn call_signal_handler_my_class_my_signal(
            handler: &mut SignalHandlerMyClassMySignal,
            arg0: &String,
        );
    }
}

use core::mem::drop as drop_signal_handler_my_class_my_signal;

fn call_signal_handler_my_class_my_signal(
    handler: &mut SignalHandler<MyClassMySignalClosure>,
    arg0: &String,
) {
    (handler.closure)(arg0);
}

struct MyClassMySignalClosure {}

impl SignalHandlerClosure for MyClassMySignalClosure {
    type Id = cxx::type_id!("test::ffi::SignalHandlerMyClassMySignal");
    type FnType = dyn FnMut(&String);
}

impl ffi::MyClass {
    pub fn on_my_signal<F: FnMut(&String) + 'static>(&self, closure: F) {
        ffi::connect_my_class_my_signal(
            self,
            SignalHandler {
                closure: Box::new(closure),
            },
        );
    }
}

// shared code
pub trait SignalHandlerClosure {
    type Id;
    type FnType: ?Sized;
}

#[repr(transparent)]
pub struct SignalHandler<T: SignalHandlerClosure> {
    closure: Box<T::FnType>,
}

impl<T: SignalHandlerClosure> Drop for SignalHandler<T> {
    fn drop(&mut self) {
        println!(
            "Dropping SignalHandler of size: {size}",
            size = std::mem::size_of::<Self>()
        );
    }
}

unsafe impl<T: SignalHandlerClosure> cxx::ExternType for SignalHandler<T> {
    type Kind = cxx::kind::Trivial;
    type Id = T::Id;
}
