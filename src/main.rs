#![no_std] // No std lib
#![no_main] // No main file required
            //
mod vga_buffer;

use core::panic::PanicInfo;

// Default _start() as entrypoint (use mostly) and no_mangle to avoid function name re-generation
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world {} {}!", "MyOS", "2026");
    println!("Welcome to my place");

    loop {}
}

// Define panic handler by our self because we on't have std lib to handle it
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
