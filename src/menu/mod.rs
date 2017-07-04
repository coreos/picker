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

use core::char;

use uefi::{SimpleTextOutput, Status};
use uefi::protocol::SerialIOProtocol;

use boot::BootOption;
use util::read_key_timeout;

fn generic_menu<'a, W, R>(
    write: W,
    read: R,
    option_a: &'a BootOption,
    option_b: &'a BootOption,
) -> Result<Option<&'a BootOption>, Status>
where
    W: Fn(&'static str),
    R: Fn() -> Result<Option<char>, Status>,
{
    loop {
        write("Option 1: ");
        write(option_a.display);
        write("\r\n");

        write("Option 2: ");
        write(option_b.display);
        write("\r\n");

        write("Enter boot choice: ");
        match read() {
            Ok(Some(key)) => {
                match key {
                    '1' => return Ok(Some(option_a)),
                    '2' => return Ok(Some(option_b)),
                    _ => write("Unrecognized option."),
                }
            }
            Ok(None) => {
                write("Taking default.\r\n");
                return Ok(None);
            }
            Err(e) => {
                write("Error reading: ");
                write(e.str());
                return Err(e);
            }
        }
        write("\r\n");
    }
}

fn serial_menu<'a>(
    serial: SerialIOProtocol,
    option_a: &'a BootOption,
    option_b: &'a BootOption,
) -> Result<Option<&'a BootOption>, Status> {
    let write = |output| {
        let cons = uefi::get_system_table().console();
        if let Err(e) = serial.write(output) {
            cons.write("Error writing to serial console: ");
            cons.write(e.str());
            cons.write("\r\n");
        }
        cons.write(output);
    };

    let read = || {
        // 50 * 100ms = 5s
        for _ in 0..50 {
            match serial.read_bytes(1) {
                Ok(Some(slice)) => {
                    let char_u8 = slice[0];

                    if char_u8 >= 0x80 {
                        return Ok(Some('?'));
                    }

                    return Ok(Some(char_u8 as char));
                }
                Ok(None) => {}
                Err(e) => return Err(e),
            }

            match read_key_timeout(1) {
                Ok(None) => {}
                e => return e,
            }
        }
        Ok(None)
    };

    generic_menu(write, read, option_a, option_b)
}

fn console_menu<'a>(
    option_a: &'a BootOption,
    option_b: &'a BootOption,
) -> Result<Option<&'a BootOption>, Status> {
    let cons = uefi::get_system_table().console();

    let write = |output| { cons.write(output); };

    let read = || read_key_timeout(5000);

    generic_menu(write, read, option_a, option_b)
}

pub fn boot_menu<'a>(
    option_a: &'a BootOption,
    option_b: &'a BootOption,
) -> Result<Option<&'a BootOption>, Status> {
    // 100000us = 100ms polling period
    uefi::protocol::SerialIOProtocol::new()
        .and_then(|mut serial| {
            serial
                .update_attributes(None, None, Some(100000), None, None, None)
                .map(|_| serial)
        })
        .and_then(|serial| serial_menu(serial, option_a, option_b))
        .or_else(|_| console_menu(option_a, option_b))
}
