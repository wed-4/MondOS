#![no_std]
#![no_main]

use core::panic::PanicInfo;

use hlt::hlt_loop;

mod hlt;
mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    // This function is called on panic
    // You can log the panic information or take other actions here
    println!("Hello World{}", "!");

    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // This is where the program starts executing
    // Initialize the system, peripherals, etc.

    loop {}
}
