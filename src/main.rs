#![no_std] // Get rid of standard library because we don't have any runtime when booting from BIOS
#![no_main] // Get rid of main because main is problematic (it presupposes that there's a stack, bad!)

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Modules
mod vga;

// Imports
use vga::Vga;

#[no_mangle] // no_mangle tells the compiler to not change this function's name
extern "C" fn _start() -> ! {
    let mut screen = Vga::new();
    screen.clear_screen();
    screen.write_str("Hello world!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

    loop {}
}