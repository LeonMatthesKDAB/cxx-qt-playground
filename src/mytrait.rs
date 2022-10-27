

trait MyTrait {
    fn myFun();
}

impl MyTrait for crate::MyObject {
    fn myFun() {
        println!("Hello world");
    }
}