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

use core::{mem, slice};

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

impl GptHeader {
    fn validate(&mut self, my_lba: u64) -> Result<(), Status> {
        let bs = uefi::get_system_table().boot_services();

        if self.signature != HEADER_SIGNATURE {
            return Err(Status::InvalidParameter);
        }

        if self.my_lba != my_lba {
            // FIXME(csssuf): is there a better error to use here? spec doesn't say
            return Err(Status::VolumeCorrupted);
        }

        let my_crc32 = self.header_crc32;

        self.header_crc32 = 0;
        let crc32 = bs.calculate_crc32_sized(self, self.header_size as usize)?;
        self.header_crc32 = my_crc32;

        if crc32 != self.header_crc32 {
            return Err(Status::CrcError);
        }

        Ok(())
    }
}

#[derive(Copy)]
#[repr(C, packed)]
pub struct GptPartitionEntry {
    pub(crate) partition_type_guid: Guid,
    pub(crate) unique_partition_guid: Guid,
    starting_lba: u64,
    ending_lba: u64,
    pub(crate) attributes: u64,
    pub(crate) partition_name: [u16; 36],
}

impl Clone for GptPartitionEntry {
    fn clone(&self) -> GptPartitionEntry {
        GptPartitionEntry {
            partition_type_guid: self.partition_type_guid,
            unique_partition_guid: self.unique_partition_guid,
            starting_lba: self.starting_lba,
            ending_lba: self.ending_lba,
            attributes: self.attributes,
            partition_name: self.partition_name,
        }
    }
}

type GptPartitionTable<'a> = &'a [&'a mut GptPartitionEntry];

pub struct GptDisk<'a> {
    block_device: &'static BlockIOProtocol,
    primary_header: &'a mut GptHeader,
    alternate_header: &'a mut GptHeader,
    raw_partitions: &'a mut [u8],
    pub partitions: GptPartitionTable<'a>,
}

impl<'a> Drop for GptDisk<'a> {
    fn drop(&mut self) {
        let bs = uefi::get_system_table().boot_services();

        self.block_device.free_read(self.raw_partitions);
        bs.free_pages(
            self.primary_header,
            self.block_device.required_pages_block(1),
        );
        bs.free_pages(
            self.alternate_header,
            self.block_device.required_pages_block(1),
        );
    }
}

impl<'a> GptDisk<'a> {
    /// Read the GPT header from a given device, and perform all necessary validation on it.
    pub fn read_from(device: &DevicePathProtocol) -> Result<GptDisk, Status> {
        let bs = uefi::get_system_table().boot_services();

        let (handle, _usable_device) = bs.locate_device_path::<BlockIOProtocol>(device)?;
        let protocol = bs.open_protocol::<BlockIOProtocol>(
            handle,
            get_current_image_handle(),
            Handle::default(),
            GET_PROTOCOL,
        )?;

        let primary_block = protocol.read_blocks(1, 1)?;
        let primary_header = unsafe { &mut *(primary_block.as_mut_ptr() as *mut GptHeader) };

        let alternate_block = protocol.read_blocks(primary_header.alternate_lba, 1)?;
        let alternate_header = unsafe { &mut *(alternate_block.as_mut_ptr() as *mut GptHeader) };

        let mut out = GptDisk {
            block_device: protocol,
            primary_header: primary_header,
            alternate_header: alternate_header,
            raw_partitions: &mut [],
            partitions: &[],
        };

        unsafe { out.validate_headers()? };

        out.read_partitions()?;

        unsafe { out.validate_partitions()? };

        Ok(out)
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
    unsafe fn validate_headers(&mut self) -> Result<(), Status> {
        // The primary header is always read from LBA 1, so we call `validate` with that value.
        self.primary_header.validate(1)?;
        self.alternate_header
            .validate(self.primary_header.alternate_lba)?;

        Ok(())
    }

    unsafe fn validate_partitions(&mut self) -> Result<(), Status> {
        let partition_crc32 = uefi::get_system_table()
            .boot_services()
            .calculate_crc32_sized(self.raw_partitions.as_ptr(), self.raw_partitions.len())?;

        if partition_crc32 != self.primary_header.partition_entry_array_crc32 {
            return Err(Status::CrcError);
        }

        Ok(())
    }

    /// Read the partition entry array from this disk and return it.
    fn read_partitions(&mut self) -> Result<(), Status> {
        let bs = uefi::get_system_table().boot_services();

        let num_partitions = self.primary_header.num_partition_entries as usize;
        self.read_raw_partitions()?;

        let entries_ptr = bs.allocate_pool::<&mut GptPartitionEntry>(
            num_partitions * mem::size_of::<&mut GptPartitionEntry>(),
        )?;

        let entries = unsafe { slice::from_raw_parts_mut(entries_ptr, num_partitions) };
        for part_number in 0..(self.primary_header.num_partition_entries) {
            let offset = (part_number * self.primary_header.sizeof_partition_entry) as isize;

            unsafe {
                let entry_ptr = self.raw_partitions.as_ptr().offset(offset);
                let entry = &mut *(entry_ptr as *mut GptPartitionEntry);
                (*entries)[part_number as usize] = entry;
            }
        }

        self.partitions = entries;
        Ok(())
    }

    fn read_raw_partitions(&mut self) -> Result<(), Status> {
        let read_size = (self.primary_header.num_partition_entries *
            self.primary_header.sizeof_partition_entry) as usize;
        self.raw_partitions = self.block_device
            .read_bytes(self.primary_header.partition_entry_lba, read_size)?;
        Ok(())
    }
}
