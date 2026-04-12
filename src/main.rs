#![no_std] // No std lib
#![no_main] // No main file required
            //
mod vga_buffer;

use core::panic::PanicInfo;

// Default _start() as entrypoint (use mostly) and no_mangle to avoid function name re-generation
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// Define panic handler by our self because we on't have std lib to handle it
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
