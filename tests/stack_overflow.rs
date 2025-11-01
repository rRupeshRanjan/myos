#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use myos::serial_print;
use myos::{QemuExitCode, exit_qemu, serial_println};
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    serial_print!("stack_overflow::stack_overflow...\t");

    myos::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed

    // we perform an additional volatile read using the Volatile type to prevent a
    // compiler optimization called tail call elimination. Among other things, this
    // optimization allows the compiler to transform a function whose last statement
    // is a recursive function call into a normal loop. Thus, no additional stack frame
    // is created for the function call, so the stack usage remains constant.
    // In our case, however, we want the stack overflow to happen, so we add a dummy volatile
    // read statement at the end of the function, which the compiler is not allowed to remove.
    // Thus, the function is no longer tail recursive, and the transformation into a loop
    // is prevented.
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myos::test_panic_handler(info)
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(myos::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
