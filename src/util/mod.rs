// Copyright 2017 CoreOS, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate uefi;

mod input;

pub use self::input::*;

use core::fmt::Arguments;

#[lang = "eh_personality"]
#[no_mangle]
pub fn rust_eh_personality() {}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub fn rust_eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub fn rust_begin_panic(_fmt: Arguments, _file_line: &(&'static str, u32)) -> ! {
    loop {}
}
