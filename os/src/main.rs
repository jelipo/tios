#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(llvm_asm)]

use core::panic::PanicInfo;

use crate::sbi::{sbi_call, sbi_ext_call, SbiError};

mod console;
mod sbi;
mod batch;
mod trap;
mod fs;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    sys_info!("Hello World!!!");
    sys_warn!("Hello World!!!");
    sys_error!("Hello World!?");

    //let i = get_spec_version();
    //println!("{:?}", i);

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
    println!("{:x}", sbss as usize);
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

const SBI_SHUTDOWN: usize = 8;

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    unreachable!()
}