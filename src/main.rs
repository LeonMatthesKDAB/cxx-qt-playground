use playground;
fn main() {
    let my_class = playground::ffi::create_my_class();

    unsafe {
        (*my_class).on_my_signal(|integer| println!("Hello {integer}"));
    }
}
