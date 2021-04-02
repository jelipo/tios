#![no_std]
#![no_main]
#![feature(llvm_asm)]

use core::panic::PanicInfo;

use crate::sbi::sbi_call;

mod console;
mod sbi;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const SBI_SHUTDOWN: usize = 8;

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

#[no_mangle]
extern "C" fn _start() {
    shutdown();
}

