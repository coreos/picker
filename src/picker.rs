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

use uefi::{Guid, Handle, SimpleTextOutput, Status};
use uefi::util::parent_device_path;

pub mod boot;
pub mod menu;
pub mod util;
pub mod uefi_entry;

use boot::BootOption;

const BOOTPATH_USR_A: &'static str = "\\EFI\\coreos\\shim_a.efi";
const BOOTPATH_USR_B: &'static str = "\\EFI\\coreos\\shim_b.efi";

const PART_UUID_USR_A: Guid = Guid(
    0x7130_C94A,
    0x213A,
    0x4E5A,
    [0x8E, 0x26, 0x6C, 0xCE, 0x96, 0x62, 0xF1, 0x32],
);
const PART_UUID_USR_B: Guid = Guid(
    0xE03D_D35C,
    0x7C2D,
    0x4A47,
    [0xB3, 0xFE, 0x27, 0xF1, 0x57, 0x80, 0xA5, 0x7C],
);

pub fn efi_main(image_handle: Handle) -> Status {
    let sys_table = uefi::get_system_table();
    let bs = sys_table.boot_services();
    let cons = sys_table.console();

    cons.write("picker v0.0.1\r\n");

    let mut option_a = BootOption {
        display: "USR-A",
        boot_data: BOOTPATH_USR_A,
        default: false,
        guid: PART_UUID_USR_A,
    };
    let mut option_b = BootOption {
        display: "USR-B",
        boot_data: BOOTPATH_USR_B,
        default: false,
        guid: PART_UUID_USR_B,
    };

    let this = uefi::protocol::get_current_image();
    let partition = bs.handle_protocol::<uefi::protocol::DevicePathProtocol>(this.device_handle)
        .and_then(parent_device_path)
        .and_then(|parent_path| util::GptDisk::read_from(parent_path))
        .and_then(|disk| disk.read_partitions().map(util::gptprio::next));

    match partition {
        Ok(Some(gptprio_partition)) => {
            if option_a.guid == gptprio_partition.unique_partition_guid {
                option_a.default = true;
            } else if option_b.guid == gptprio_partition.unique_partition_guid {
                option_b.default = true;
            } else {
                cons.write(
                    "Unknown gptprio partition chosen as next. Defaulting to USR-A.\r\n",
                );
                option_a.default = true;
            }
        }
        Ok(None) => {
            cons.write(
                "No acceptable gptprio partitions found; defaulting to USR-A.\r\n",
            );
            option_a.default = true;
        }
        Err(e) => {
            cons.write("error reading from disk: ");
            cons.write(e.str());
            return e;
        }
    }

    let boot_result = menu::boot_menu(&option_a, &option_b)
        .and_then(|option| {
            match option {
                Some(boot_choice) => Ok(boot_choice),
                None => {
                    cons.write(
                        "No option selected and no default was set. Can't proceed.\r\n",
                    );
                    // FIXME(csssuf) is this the best error to use here?
                    Err(Status::NoMedia)
                }
            }
        })
        .and_then(|boot_option| boot::boot(boot_option, image_handle));

    match boot_result {
        Ok(_) => Status::Success,
        Err(e) => e,
    }
}
