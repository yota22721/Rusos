use crate::start::start;
use core::arch::asm;

#[link_section = ".entry"]
#[no_mangle]

pub unsafe extern "C" fn _entry() -> ! {
    asm!(
      "la sp, STACK0",
      "li a0, 4096 * {ssz}",
      "csrr a1, mhartid",
      "addi a1, a1, 1",
      "mul a0, a0, a1",
      "add sp, sp, a0",
      ssz = const STACK_PAGE_NUM,
    );

    start();
}