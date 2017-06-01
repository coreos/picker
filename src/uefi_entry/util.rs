extern crate uefi;

use core::fmt::Arguments;

extern "rust-intrinsic" {
    pub fn transmute<T, U>(val: T) -> U;
    pub fn abort() -> !;
}

#[lang = "eh_personality"]
#[no_mangle]
pub fn rust_eh_personality() {}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub fn rust_eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub fn rust_begin_panic(_fmt: Arguments, _file_line: &(&'static str, u32)) -> ! {
    unsafe {
        abort();
    }
}

// Needed for copy_nonoverlapping.
#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    uefi::get_system_table()
        .boot_services()
        .copy_mem(dest, src, n)
}

#[no_mangle]
pub extern "C" fn memset(buf: *mut u8, val: isize, size: usize) -> *const u8 {
    uefi::get_system_table()
        .boot_services()
        .set_mem(buf, val as u8, size)
}
