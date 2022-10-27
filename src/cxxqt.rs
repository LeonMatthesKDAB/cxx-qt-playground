#[cxx_qt::bridge]
mod ffi {
    #[cxx_qt::qobject(base=QAbstractItemModel)]
    #[cxx_qt::qobject]
    struct MyObject {
        #[qproperty]
        prop: i32,

        private_stuff: MySpecialType,
    }

    
    #[cxx_qt::signals(MyObject)]
    enum MySignals {
        Ready,
    }

    impl Default for MyObject {
        Self {
            prop: 42,
            private_stuff: ...,
        }
    }

    impl cxx_qt<MyObject> {
        #[invokable]
        pub fn add(self: Pin<&mut Self>) {
            let prop = self.as_ref().get_prop();
            self.as_mut().set_prop(prop + 1);

            self.as_ref().rust().thing()
        }
    }

    impl MyObject {
        fn thing(&self) {

        }
    }
}