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
#[allow(unused_variables)]
#[no_mangle]
pub fn rust_begin_panic(fmt: Arguments, file_line: &(&'static str, u32)) -> ! {
    unsafe {
        abort();
    }
}

pub fn str_to_u16_pointer(s: &str) -> *const u16 {
    let mut buf = [0u16; 4096];

    let mut i = 0;
    for c in s.chars() {
        if i >= 4096 {
            break;
        }
        buf[i] = c as u16;
        i += 1;
    }

    *buf.last_mut().unwrap() = 0;

    unsafe { transmute(&buf) }
}

// Needed for copy_nonoverlapping.
#[no_mangle]
pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i: usize = 0;
    while i < n {
        unsafe {
            *(dest.offset(i as isize)) = *(src.offset(i as isize));
        }

        i += 1;
    }

    dest
}

#[no_mangle]
pub extern "C" fn memset(buf: *mut u8, val: isize, size: usize) -> *const u8 {
    uefi::get_system_table()
        .boot_services()
        .set_mem(buf, val as u8, size)
}
