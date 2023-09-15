use playground;
fn main() {
    let my_class = playground::ffi::MyClass();

    unsafe {
        (*my_class).on_my_signal(|integer| println!("Hello {integer}"));
    }
}
