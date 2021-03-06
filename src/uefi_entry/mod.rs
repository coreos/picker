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

extern crate uefi;

use uefi::{protocol, Status};

#[no_mangle]
pub extern "win64" fn efi_entry(
    image_handle: uefi::Handle,
    system_table: *const uefi::SystemTable,
) -> isize {
    uefi::set_system_table(system_table).console().reset();

    let loaded_image_proto: Result<&'static protocol::LoadedImageProtocol, Status> =
        protocol::set_current_image(image_handle);
    if let Err(status) = loaded_image_proto {
        return status as isize;
    }

    ::efi_main(image_handle) as isize
}
