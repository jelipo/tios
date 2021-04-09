#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(llvm_asm)]

use core::panic::PanicInfo;

use crate::sbi::sbi_call;

mod console;
mod sbi;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    println!("Hello World");
    shutdown()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

const SBI_SHUTDOWN: usize = 8;

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}