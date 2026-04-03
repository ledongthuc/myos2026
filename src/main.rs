#![no_std] // No std lib
#![no_main] // No main file required

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// Default _start() as entrypoint (use mostly) and no_mangle to avoid function name re-generation
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// Define panic handler by our self because we on't have std lib to handle it
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
