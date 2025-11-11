#![no_std]
#![no_main]
use core::panic::PanicInfo;
use kernel::{kmain, print, println};
//use crate::kernel::printf::*;

kmain!(main);

extern "C" fn main() -> ! {
    //println!("");
    println!("Rusos is booting");
    //println!("");
    print!("hello\n");
    loop{}
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
