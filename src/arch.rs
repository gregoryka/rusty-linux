#![no_std]

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
mod x86_shared;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

mod mm_inner;
mod topology_inner;
