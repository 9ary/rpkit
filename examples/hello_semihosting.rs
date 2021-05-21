// SPDX-License-Identifier: CC0-1.0

#![no_std]
#![no_main]

use panic_semihosting as _;
use cortex_m_semihosting::hprintln;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    loop {}
}
