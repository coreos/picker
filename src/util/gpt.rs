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

use uefi::{Guid, Handle, Status, GET_PROTOCOL};
use uefi::protocol::{get_current_image_handle, BlockIOProtocol, DevicePathProtocol};

// "EFI PART", u64 constant given in UEFI spec
const HEADER_SIGNATURE: u64 = 0x5452_4150_2049_4645;

#[repr(C, packed)]
pub struct GptHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    header_crc32: u32,
    _reserved: u32,
    my_lba: u64,
    alternate_lba: u64,
    first_usable_lba: u64,
    last_usable_lba: u64,
    disk_guid: Guid,
    partition_entry_lba: u64,
    num_partition_entries: u32,
    sizeof_partition_entry: u32,
    partition_entry_array_crc32: u32,
}

impl Drop for GptHeader {
    fn drop(&mut self) {
        uefi::get_system_table().boot_services().free_pool(self);
    }
}

impl GptHeader {
    fn validate(&self, my_lba: u64, block_io: &'static BlockIOProtocol) -> Result<(), Status> {
        let bs = uefi::get_system_table().boot_services();

        if self.signature != HEADER_SIGNATURE {
            return Err(Status::InvalidParameter);
        }

        if self.my_lba != my_lba {
            // FIXME(csssuf): is there a better error to use here? spec doesn't say
            return Err(Status::VolumeCorrupted);
        }

        block_io
            .read_blocks(self.my_lba, 1)
            .and_then(|header_copy_block| {
                let this_header =
                    unsafe { &mut *(header_copy_block.as_mut_ptr() as *mut GptHeader) };

                this_header.header_crc32 = 0;

                bs.calculate_crc32_sized(this_header, self.header_size as usize)
                    .and_then(|crc32| {
                        if crc32 != self.header_crc32 {
                            return Err(Status::CrcError);
                        }

                        Ok(())
                    })
            })
            .and_then(|_| {
                let partition_entry_table_size =
                    (self.num_partition_entries * self.sizeof_partition_entry) as usize;

                block_io
                    .read_bytes(self.partition_entry_lba, partition_entry_table_size)
                    .and_then(|entry_table| {
                        bs.calculate_crc32_sized(entry_table.as_ptr(), partition_entry_table_size)
                            .and_then(|crc32| {
                                if crc32 != self.partition_entry_array_crc32 {
                                    return Err(Status::CrcError);
                                }

                                Ok(())
                            })
                    })
            })
    }
}

pub struct GptDisk<'a> {
    block_device: &'static BlockIOProtocol,
    primary_header: &'a GptHeader,
    alternate_header: &'a GptHeader,
}

impl<'a> GptDisk<'a> {
    /// Read the GPT header from a given device, and perform all necessary validation on it.
    pub fn read_from(device: &DevicePathProtocol) -> Result<GptDisk, Status> {
        let bs = uefi::get_system_table().boot_services();

        bs.locate_device_path::<BlockIOProtocol>(device)
            .and_then(|(handle, usable_device)| {
                bs.open_protocol::<BlockIOProtocol>(
                    handle,
                    get_current_image_handle(),
                    Handle::default(),
                    GET_PROTOCOL,
                ).and_then(|protocol| {
                        // Read 1 block starting at block 1.
                        protocol.read_blocks(1, 1).and_then(|block| {
                            let mut out = GptDisk {
                                block_device: protocol,
                                primary_header: unsafe { &*(block.as_ptr() as *const GptHeader) },
                                alternate_header: unsafe { &*(block.as_ptr() as *const GptHeader) },
                            };

                            unsafe {
                                out.validate().map(|_| out).map_err(|e| {
                                    bs.free_pool(block.as_ptr());
                                    e
                                })
                            }
                        })
                    })
            })
    }

    /// Validate an instance of a `GptHeader`.
    /// The UEFI spec gives the steps required to validate a GPT header as follows:
    /// >* Check the Signature
    /// >* Check the Header CRC
    /// >* Check that the MyLBA entry points to the LBA that contains the GUID Partition Table
    /// >* Check the CRC of the GUID Partition Entry Array
    ///
    /// >If the GPT is the primary table, stored at LBA 1:
    ///
    /// >* Check the AlternateLBA to see if it is a valid GPT
    unsafe fn validate(&mut self) -> Result<(), Status> {
        // The primary header is always read from LBA 1, so we call `validate` with that value.
        self.primary_header
            .validate(1, self.block_device)
            .and_then(|_| {
                self.block_device
                    .read_blocks(self.primary_header.alternate_lba, 1)
                    .and_then(|block| {
                        self.alternate_header = &*(block.as_ptr() as *const GptHeader);
                        self.alternate_header
                            .validate(self.primary_header.alternate_lba, self.block_device)
                    })
            })
    }
}
