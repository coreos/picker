#![no_std]
#![feature(intrinsics)]
#![feature(lang_items)]

extern crate uefi;
use uefi::*;

pub mod uefi_entry;

pub fn efi_main() -> isize {
    let cons = uefi::get_system_table().console();
    cons.write("picker v0.0.1\r\n");
    0
}
