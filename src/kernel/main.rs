#![no_std]
#![no_main]
use core::panic::PanicInfo;
use kernel::{printf};
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub unsafe extern "C" fn  main() -> ! {
    println!("");
    println!("Rusos is booting");
    println!("");
    loop{}
}