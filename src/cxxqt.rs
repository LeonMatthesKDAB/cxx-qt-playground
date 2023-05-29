#[cxx_qt::bridge]
mod ffi {
    extern "Qt" {
        type QAbstractItemModel = cxx_qt_lib::QAbstractItemModel;

        type OtherObject = crate::qobject::OtherObject;

        type QButton;

        #[qsignal]
        fn clicked(self: &QButton);
    }
    
    #[cxx_qt::qobject(base=QAbstractItemModel, qml_element)]
    struct MyObject {
        #[qproperty(read="get_my_prop",write="set_my_prop")]
        prop: i32,

        private_stuff: MySpecialType,
    }

    impl qobject::MyObject {
        #[qsignal]
        #[inherit]
        fn data(self: Pin<&mut Self>, my_thing: &MyType) {}

        #[cxx_name = "hasChildren"]
        #[inherit]
        fn has_children_super(&self, parent: &QModelIndex) -> bool {}

        fn get_my_prop(&self) -> i32 {
            todo!{}
        }

        fn set_my_prop(self: Pin<&mut Self>, value: i32) {
            todo!{}
        }

        #[invokable]
        #[return_cxx_type="Result<String>"]
        pub fn my_invokable(self: Pin<&mut Self>) ->  anyhow::Result<()> {
            todo! { }
        }
    }

    impl cxx_qt::Constructor<(i32)> for qobject::MyObject {
        todo!{}
    }

    impl cxx_qt::Threading for MyObject {}
    // to disable CXXQt locking!
    impl !cxx_qt::Locking for MyObject {}
}