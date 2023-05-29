// cxx-qt crate
mod ffi {
    extern "C++" {
        type QObject;

        fn children(self: &QObject) -> QList<*mut QObject>;
        fn objectName(self: &QObject) -> QString;
    }
}

trait QtObject {
    fn as_qobject(&self) -> &cxx_qt_lib::QObject;
    fn as_qobject_mut(&mut self) -> Pin<&mut cxx_qt_lib::QObject>;

    fn children(&self) -> QList<*mut QObject> {
        as_qobject().children()
    }

    fn objectName(&self) -> Pin<&mut cxx_qt_lib::QObject> {
        as_qobject().objectName()
    }
}

trait CxxQtObject: QtObject {
    type Inner;
    
    /// 
    fn rust(&self) -> Self::Inner;
    fn rust_mut(self: Pin<&mut self>) -> &mut Self::Inner;
}

trait Threading {
    fn qt_thread() -> Threading<Self>;
}
