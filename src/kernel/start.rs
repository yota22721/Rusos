use core::hint::unreachable_unchecked;
#[no_mangle]
pub unsafe fn start() -> ! {
    extern "C" {
        fn main() -> !;
    }
    unreachable_unchecked();
}
