#![no_std] // dont link Rust standard library
#![no_main] // disable all Rust leevle entry points
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use myos::println;

// This function is called on panic
#[cfg(not(test))]
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

    myos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash");

    loop {
        use myos::print;
        print!("-");
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
