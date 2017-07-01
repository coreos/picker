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

use uefi::*;

fn str_to_device_path(image: &str) -> Result<&protocol::DevicePathProtocol, Status> {
    let bs = uefi::get_system_table().boot_services();
    bs.locate_protocol::<protocol::DevicePathFromTextProtocol>(0 as *const CVoid)
        .and_then(|from_text| from_text.text_to_device_path_node(image))
}

fn build_boot_path(
    file: &protocol::DevicePathProtocol,
) -> Result<*const protocol::DevicePathProtocol, Status> {
    let bs = uefi::get_system_table().boot_services();

    bs.handle_protocol::<protocol::DevicePathProtocol>(protocol::get_current_image().device_handle)
        .and_then(|this_device_path| {
            bs.locate_protocol::<protocol::DevicePathUtilitiesProtocol>(0 as *const CVoid)
                .and_then(|utilities| {
                    utilities.append_device_node(this_device_path, file).map(
                        |output| output as *const protocol::DevicePathProtocol,
                    )
                })
        })
}

pub fn boot_image(image: &str, parent: Handle) -> Result<(), Status> {
    let bs = uefi::get_system_table().boot_services();

    str_to_device_path(image)
        .and_then(build_boot_path)
        .and_then(|full_path| {
            bs.load_image(true, parent, full_path).and_then(
                |loaded_image| {
                    bs.start_image(loaded_image)
                },
            )
        })
}
