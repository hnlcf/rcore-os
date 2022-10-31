//! Re-implement the system panic
use crate::{println, sbi};
use core::panic::PanicInfo;

/// Custom implementation of `panic!` macro
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }

    sbi::shutdown()
}
