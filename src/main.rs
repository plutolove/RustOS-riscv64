#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_item;
mod sbi;

use core::arch::asm;
use core::fmt::{self, Write};

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
extern "C" fn rust_main() {
    clear_bss();
    println!("Hello world!");
    sbi::shutdown();
}
