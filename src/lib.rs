use std::{ffi::*, pin::Pin};

#[cxx::bridge(namespace = "my_library")]
pub mod ffi {
    unsafe extern "C++" {
        include!("/home/kdab/Documents/projects/3371-Rust-RnD/playground/src/myobject.h");
        type SignalHandlerMyClassMySignal = super::SignalHandler<super::MyClassMySignalParams>;

        #[cxx_name = "connect_MyClass_mySignal"]
        fn connect_my_class_my_signal(obj: &MyClass, signal_handler: SignalHandlerMyClassMySignal);

        fn create_my_class() -> *mut MyClass;

        type MyClass;
    }

    #[qml_element]
    qnamespace!("my_library::my_library");

    #[qenum]
    enum MyEnum {
        A,
        B,
    }

    extern "Rust" {
        unsafe fn drop_signal_handler_my_class_my_signal(handler: SignalHandlerMyClassMySignal);

        fn call_signal_handler_my_class_my_signal(
            handler: &mut SignalHandlerMyClassMySignal,
            arg0: i32,
        );
    }
}

use core::mem::drop as drop_signal_handler_my_class_my_signal;

fn call_signal_handler_my_class_my_signal(
    handler: &mut SignalHandler<MyClassMySignalParams>,
    arg0: i32,
) {
    (handler.closure)((arg0,));
}

struct MyClassMySignalParams {}

impl SignalHandlerParameters for MyClassMySignalParams {
    type Id = cxx::type_id!("test::ffi::SignalHandlerMyClassMySignal");
    type Arguments = (i32,);
}

impl ffi::MyClass {
    pub fn on_my_signal<F: FnMut(i32) + 'static>(&self, mut closure: F) {
        ffi::connect_my_class_my_signal(
            self,
            SignalHandler {
                closure: Box::new(move |args| closure(args.0)),
            },
        );
    }
}

// shared code
pub trait SignalHandlerParameters {
    type Id;
    type Arguments;
}

#[repr(transparent)]
pub struct SignalHandler<T: SignalHandlerParameters> {
    closure: Box<dyn FnMut(T::Arguments)>,
}

impl<T: SignalHandlerParameters> Drop for SignalHandler<T> {
    fn drop(&mut self) {
        println!(
            "Dropping SignalHandler of size: {size}",
            size = std::mem::size_of::<Self>()
        );
    }
}

unsafe impl<T: SignalHandlerParameters> cxx::ExternType for SignalHandler<T> {
    type Kind = cxx::kind::Trivial;
    type Id = T::Id;
}
