// defined in library
pub trait CxxQtRustObj {
    type FFIRef;
    type FFI;
}

pub mod qt {
    use std::ops::Deref;
    use std::ops::DerefMut;

    // defined in library
    pub struct Qt<T>
    where
        T: super::CxxQtRustObj,
    {
        rust: T,
        cpp: T::FFIRef,
    }

    impl<T> Qt<T>
    where
        T: super::CxxQtRustObj,
    {
        pub fn new(rust: T, cpp: T::FFIRef) -> Self {
            Self { rust, cpp }
        }
    }

    impl<T> Qt<T>
    where
        T: super::CxxQtRustObj,
    {
        pub fn cpp(self) -> T::FFIRef {
            self.cpp
        }

        pub fn rust(self) -> T {
            self.rust
        }
    }

    use std::borrow::BorrowMut;
    impl<'a, T> Qt<&'a mut T>
    where
        &'a mut T: super::CxxQtRustObj,
        <&'a mut T as super::CxxQtRustObj>::FFIRef:
            BorrowMut<<&'a mut T as super::CxxQtRustObj>::FFI>,
    {
        pub unsafe fn rust_mut(&mut self) -> &mut T {
            self.rust
        }

        pub unsafe fn cpp_mut(&mut self) -> &mut <&'a mut T as super::CxxQtRustObj>::FFI {
            self.cpp.borrow_mut()
        }
    }

    impl<T> Clone for Qt<T>
    where
        T: super::CxxQtRustObj + Clone,
        <T as super::CxxQtRustObj>::FFIRef: Clone,
    {
        fn clone(&self) -> Self {
            Self {
                rust: self.rust.clone(),
                cpp: self.cpp.clone(),
            }
        }
    }

    impl<T> Copy for Qt<T>
    where
        T: super::CxxQtRustObj + Copy,
        <T as super::CxxQtRustObj>::FFIRef: Copy,
    {
    }
}
