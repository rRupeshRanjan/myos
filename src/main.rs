#![no_std] // dont link Rust standard library
#![no_main] // disable all Rust leevle entry points

mod vga_buffer;
use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[unsafe(no_mangle)] // dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a fucntion
    // named _start by default

    println!("Hello world{}", "!");

    loop {}
}
