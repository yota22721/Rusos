use crate::param::*;

#[no_mangle]
static mut STACK0:u8 = [4096 * STACK_PAGE_NUM * NCPU];

pub unsafe fn start() -> ! {
    extern "C" {
        fn main() -> !;
    }
}