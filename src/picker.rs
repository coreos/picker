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
#![feature(lang_items)]

#[macro_use]
extern crate bitfield;
extern crate rlibc;
extern crate uefi;

use uefi::*;

pub mod boot;
pub mod menu;
pub mod util;
pub mod uefi_entry;

use boot::BootOption;

const BOOTPATH_1: &'static str = "\\efi\\boot\\shim_a.efi";
const BOOTPATH_2: &'static str = "\\efi\\boot\\shim_b.efi";

pub fn efi_main(image_handle: Handle) -> Status {
    let sys_table = uefi::get_system_table();
    let cons = sys_table.console();

    cons.write("picker v0.0.1\r\n");

    let option_a = BootOption {
        display: "application a",
        boot_data: BOOTPATH_1,
    };
    let option_b = BootOption {
        display: "application b",
        boot_data: BOOTPATH_2,
    };

    match menu::boot_menu(&option_a, &option_b).and_then(|option| {
        let result = option.unwrap_or(&option_a);
        boot::boot(result, image_handle)
    }) {
        Ok(_) => Status::Success,
        Err(e) => e,
    }
}
