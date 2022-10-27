// defined in CXX-qt-lib
/// 
#[repr(transparent)]
pub struct Value<T> {
    value: UniquePtr<T>
}

impl From<UniquePtr<T>> for Value<T>
{
}

impl Into<UniquePtr<T>> for Value<T>

#[cxxqt::bridge]
mod ffi {
    extern "Qt" {
        struct MyObject {
            #[prop]
            prop: i32,
            #[prop]
            color: UniquePtr<QColor>
            #[prop]
            string: Value<QString>
            // #[prop(read=pointer)]
            // another_obj: UniquePtr<MyDataObject>
        }

        std::unique_ptr<QColor> getter() {
        }

        impl MyObject {
            fn get_string(&self) -> UniquePtr<QString> {
                self.string.clone();
            }

            fn take_string()
            fn give_string()
        }

        QString getter() {
            return std::move(*rust->get_string()); // Return UniquePtr<QString>
            return m_string;
        }

        impl MyObjectQt {
            #[invokable]
            pub fn add(self: Pin<&mut Self>) {
                let prop = self.as_ref().get_prop();
                self.as_mut().set_prop(prop + 1);
            }

            #[invokable]
            pub fn my_invokable(self: Pin<&mut Self>,
                color: &QColor,
                color: Pin<&mut QColor>,
                color: UnqiuePtr<QColor>) -> Value<QColor> {
                let prop = self.as_ref().get_prop();
                self.as_mut().set_prop(prop + 1);

                return Value::from(QColorUnique);
                return UniquePtr::new(QCOlorUnique).into()
            }
        }
    }

    // CppToValue< UniquePtr< S > >  ... S become C++ type
    QColor add(const QColor& color, QColor& color) {
        static_assert!()
        return std::move(*add().release());
    }

    // extern "C++" {
    //     type MyObjectCpp;

    //     fn my_thing(self: &MyObjectCpp) -> bool;
    // }
}
