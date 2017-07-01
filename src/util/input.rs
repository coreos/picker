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

use core::ptr;

use uefi::*;

pub fn read_key_timeout(timeout_ms: u64) -> Result<Option<InputKey>, Status> {
    let st = uefi::get_system_table();
    let bs = st.boot_services();
    let cons = st.console();

    let event_result = bs.create_event(EventType::Timer, TPL::Application, None, ptr::null());
    event_result.and_then(|timer_event| {
        let events: [Event; 2] = [cons.wait_for_key(), timer_event];

        let set_result = bs.set_timer(timer_event, TimerDelay::Relative, timeout_ms * 10000);
        if set_result != Status::Success {
            return Err(set_result);
        }

        bs.wait_for_event(&events).and_then(|event| match event {
            0 => cons.read_key_async().and_then(|key| Ok(Some(key))),
            _ => Ok(None),
        })
    })
}
