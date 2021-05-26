// SPDX-License-Identifier: CC0-1.0

#![no_std]
#![no_main]

#![deny(unsafe_op_in_unsafe_fn)]

use panic_semihosting as _;
use cortex_m_semihosting::hprintln;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    let pac = rp2040_pac::Peripherals::take().unwrap();

    pac.CLOCKS.clk_sys_resus_ctrl.write(|w| w.enable().clear_bit());

    pac.CLOCKS.clk_ref_ctrl.write(|w| w.src().rosc_clksrc_ph());
    while pac.CLOCKS.clk_ref_selected.read().bits() != 1 << 0 {}
    pac.CLOCKS.clk_sys_ctrl.write(|w| w.src().clk_ref());
    while pac.CLOCKS.clk_sys_selected.read().bits() != 1 << 0 {}

    unsafe { rpkit::xosc::init(pac.XOSC); }
    hprintln!("XOSC initialized successfully!").unwrap();

    unsafe {
        pac.RESETS.reset.clear_bits(|w| w.pll_sys().clear_bit());
    }
    while pac.RESETS.reset_done.read().pll_sys().bit_is_clear() {}
    unsafe { rpkit::pll::init(pac.PLL_SYS, 1, 125, 6, 2); }
    hprintln!("PLL initialized successfully!").unwrap();

    pac.CLOCKS.clk_ref_ctrl.write(|w| w.src().xosc_clksrc());
    while pac.CLOCKS.clk_ref_selected.read().bits() != 1 << 2 {}
    hprintln!("clk_ref now clocked from XOSC!").unwrap();

    pac.CLOCKS.clk_sys_ctrl.write(|w| w.auxsrc().clksrc_pll_sys());
    pac.CLOCKS.clk_sys_ctrl.write(|w| w.src().clksrc_clk_sys_aux());
    while pac.CLOCKS.clk_sys_selected.read().bits() != 1 << 1 {}
    hprintln!("clk_sys now clocked from PLL!").unwrap();

    loop {}
}
