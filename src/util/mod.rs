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
