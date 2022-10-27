// where cxx_qt::QObject is a trait that links T to qobject::T
trait Constructor<T> {
    fn construct(args: T) -> Self;
}
struct MyStruct {
    x: i32,
    y: String,
}
impl Constructor<(i32, String)> for MyStruct {
    fn construct((num, string): (i32, String)) -> Self {
        MyStruct { x: num, y: string }
    }
}

fn main() {
    let my = <MyStruct as Constructor<(i32, String)>>::construct((5, "hello".to_owned()));
    println!("MyStruct: {}, {}", my.x, my.y);
}
