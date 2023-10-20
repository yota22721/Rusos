use crate::start::start;
use core::arch::asm;

#[link_section = ".entry"]
#[no_mangle]

pub unsafe extern "C" fn _entry() -> ! {
    asm!(
      "la sp, STACK0",
    );

    start();
}