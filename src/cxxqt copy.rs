#[cxxqt::bridge]
mod qobject {
    extern "Qt" {
        type QAbstractItemModel = cxx_qt_lib::QAbstractItemModel;

        type QButton;

        type OtherObject = crate::qobject::OtherObject;

        #[qsignal]
        fn clicked(self: &QButton);
    }

    extern "Rust" {
        #[rust_name="MyObject"]
        type MyObjectRust;
    }
    extern "RustQt" {
        #[base="QAbstractItemModel"]
        #[qml_element]
        #[qproperty(i32, member="prop")] // needs to access field .prop
        #[qproperty(i32, read="get_my_prop", write="set_my_prop")] //#[qproperty(i32, member="prop")]
        #[qproperty(QString, read="get_my_prop", write="set_my_prop")] //#[qproperty(QString, member="prop")]
        type MyObject = super::MyObject;

        #[qinvokable]
        fn my_invokable(self: Pin<&mut Self>, ..., *mut OtherObject) -> Result<()>;
    
       #[qsignal]
       #[inherit]
        fn data(self: Pin<&mut MyObject>, ...) -> ...;

        #[cxx_name = "hasChildren"]
        #[inherit]
        fn has_children_super(self: &MyObject, parent: &QModelIndex) -> bool;
    }

    impl cxx_qt::Constructor<(i32)> for MyObject {
        todo!{}
    }

    impl cxx_qt::Threading for MyObject {}
    // to disable CXXQt locking!
    impl !cxx_qt::Locking for MyObject {}
}

struct MyObject {
    pub prop: i32,

    private_stuff: MySpecialType,
}

impl qobject::MyObject {
    fn get_my_prop(&self) -> i32 {
        todo!{}
    }
    
    fn set_my_prop(self: Pin<&mut Self>, value: i32) {
        todo!{}
    }

    pub fn my_invokable(self: core::pin::Pin<&mut Self>) ->  anyhow::Result<()> {
        todo! { }
    }
}