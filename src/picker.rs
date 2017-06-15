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
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]
#![feature(intrinsics)]
#![feature(lang_items)]

extern crate rlibc;
extern crate uefi;

use uefi::*;

pub mod util;

pub mod uefi_entry;

pub fn efi_main() -> Status {
    let cons = uefi::get_system_table().console();
    cons.write("picker v0.0.1\r\n");
    Status::Success
}
