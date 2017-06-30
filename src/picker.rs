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

const BOOTPATH_1: &'static str = "\\efi\\boot\\shim_a.efi";
const BOOTPATH_2: &'static str = "\\efi\\boot\\shim_b.efi";

pub fn efi_main(image_handle: Handle) -> Status {
    let sys_table = uefi::get_system_table();
    let cons = sys_table.console();

    cons.write("picker v0.0.1\r\n");

    loop {
        cons.write("Option 1: ");
        cons.write(BOOTPATH_1);
        cons.write("\r\n");

        cons.write("Option 2: ");
        cons.write(BOOTPATH_2);
        cons.write("\r\n");

        cons.write("Option (taking default in 5 seconds...): ");
        match util::read_key_timeout(5000) {
            Ok(Some(key)) => {
                let output: [u16; 2] = [key.unicode_char, 0];
                cons.write_raw(&output as *const u16);
                cons.write("\r\n");

                match key.unicode_char as u8 as char {
                    '1' => {
                        if let Err(e) = util::boot_image(BOOTPATH_1, image_handle) {
                            cons.write("Couldn't boot choice: ");
                            cons.write(e.str());
                            cons.write("\r\n");
                            return e;
                        }
                        break;
                    }
                    '2' => {
                        if let Err(e) = util::boot_image(BOOTPATH_2, image_handle) {
                            cons.write("Couldn't boot choice: ");
                            cons.write(e.str());
                            cons.write("\r\n");
                            return e;
                        }
                        break;
                    }
                    _ => {
                        cons.write("Unrecognized option.");
                    }
                }
            }
            Ok(None) => {
                cons.write("\r\nTaking default.\r\n");
                if let Err(e) = util::boot_image(BOOTPATH_1, image_handle) {
                    cons.write("Couldn't boot default: ");
                    cons.write(e.str());
                    cons.write("\r\n");
                    return e;
                }
                break;
            }
            Err(_) => {
                cons.write("\r\nCouldn't read key.");
            }
        }

        cons.write("\r\n");
    }

    Status::Success
}
