// SPDX-License-Identifier: MPL-2.0

use crate::pac;

pub struct SystemConfig {
    peripherals: pac::Peripherals,
}

impl SystemConfig {
    pub fn new() -> Option<Self> {
        Some(SystemConfig {
            peripherals: pac::Peripherals::take()?,
        })
    }

    pub fn bootstrap(self) {
    }
}
