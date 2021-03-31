#![no_std]
#![no_main]
#![feature(llvm_asm)]

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() {
    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}