#![cfg_attr(target_os = "none", no_std)]
#![feature(asm_const)]

#[cfg(all(target_os = "none", feature = "kernel"))]
pub mod param;
#[cfg(all(target_os = "none", feature = "kernel"))]
pub mod entry;
#[cfg(all(target_os = "none", feature = "kernel"))]
pub mod start;
#[cfg(all(target_os = "none", feature = "kernel"))]
#[macro_use]
pub mod printf;

#[macro_export]
macro_rules! kmain {
    ($path:path) => {
        #[export_name = "main"]
        pub extern "C" fn __main() -> ! {
            // type check the given path
            let f: extern "C" fn() -> ! = $path;

            f()
        }
    };
}
