struct MyStruct {
    x: i32,
}
#[allow(non_snake_case)]
mod QObject {
    pub struct MyStruct {
        pub y: i32,
    }
}

fn main() {
    let my = MyStruct { x: 5 };

    let other = QObject::MyStruct { y: 10 };

    println!("MyStruct: {}", my.x);
    println!("QObject::MyStruct: {}", other.y);
}
