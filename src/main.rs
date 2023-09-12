#![no_std] // Get rid of standard library because we don't have any runtime when booting from BIOS
#![no_main] // Get rid of main because main is problematic (it presupposes that there's a stack, bad!)

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// fn main() implies that a stack will be setup before calling the function
// we don't have a stack when we start a kernel, we need to set it up

// there's a difference between Rust calling convention and C calling convention
// we need to use the C calling convention, with extern "C"

// b"some string" creates &[u8]
// "some string" creates &str
static HELLO: &[u8] = b"Welcome to NikOS!";

#[no_mangle] // no_mangle tells the compiler to not change this function's name
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x40; // 4 means red, 0 means black
        }
    }

    loop {}
}