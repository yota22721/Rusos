#![cfg_attr(target_os = "none", no_std)]

#[cfg(all(target_os = "none", feature = "kernel"))]
pub mod printf;