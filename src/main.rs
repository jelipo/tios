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
    println!("\x1b[31mhello world\x1b[0m {}", "2021-04-15");
    shutdown()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    shutdown()
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
    unreachable!()
}