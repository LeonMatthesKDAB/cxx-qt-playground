use evdev::Device;
use signal_hook::consts::TERM_SIGNALS;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn process_events(device: &mut Device) -> std::io::Result<()> {
    let events = device.fetch_events()?;
    for ev in events {
        println!("{:?}", ev);
    }
    Ok(())
}

fn grab_inputs(device: &mut Device) {
    let terminate = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        signal_hook::flag::register(*sig, Arc::clone(&terminate)).unwrap();
    }

    let mut device = Device::open("/dev/input/event4").unwrap();

    std::thread::spawn(move || {
        device.grab().unwrap();
        loop {
            if let Err(e) = process_events(&mut device) {
                eprintln!("Error: {}", e);
            }
        }
    });

    while !terminate.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}

fn main() {
    for (path, device) in evdev::enumerate() {
        println!(
            "{}, {}",
            path.to_string_lossy(),
            device.name().unwrap_or("Unknown Device")
        );
    }
}
