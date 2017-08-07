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

use util::gpt::GptPartitionEntry;

use uefi::Guid;

static COREOS_ROOTFS_TYPE: Guid = Guid(
    0x5DFB_F5F4,
    0x2848,
    0x4BAC,
    [0xAA, 0x5E, 0x0D, 0x9A, 0x20, 0xB7, 0x45, 0xA6],
);

bitfield! {
    struct GptprioAttributes(u64);
    _required, _: 0;
    _no_block_io, _: 1;
    _legacy_bios_bootable, _: 2;
    _unused1, _: 47, 3;
    gptprio_priority, _: 51, 48;
    gptprio_tries_left, _: 55, 52;
    gptprio_successful, _: 56;
    _unused2, _: 63, 57;
}

pub fn next(partitions: &[&mut GptPartitionEntry]) -> Option<GptPartitionEntry> {
    let partition_iter = partitions.into_iter();

    partition_iter
        .filter(|partition| {
            partition.partition_type_guid == COREOS_ROOTFS_TYPE
        })
        .filter(|partition| {
            let attributes = GptprioAttributes(partition.attributes);
            attributes.gptprio_successful() || attributes.gptprio_tries_left() > 0
        })
        .max_by_key(|partition| {
            GptprioAttributes(partition.attributes).gptprio_priority()
        })
        .map(|part| **part)
}
