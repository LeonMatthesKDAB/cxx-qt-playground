#[cxxqt::bridge]
mod ffi {
    extern "Qt" {
        struct MyObject {
            #[prop]
            prop: i32,

            private_stuff: MyCustomRustType
        }

        impl Default for MyObject {
            Self {
                prop: 42,
            }
        }

        impl MyObject {
            fn thing(&self) {

            }
        }

        // Qt<T> is just syntactic sugar, the type doesn't do anything.
        impl Qt<MyObject> {
            #[invokable]
            pub fn add(self: Pin<&mut Self>) {
                let prop = self.as_ref().get_prop();
                self.as_mut().set_prop(prop + 1);

                self.as_ref().rust().thing()
            }
        }
    }
}
