// SPDX-License-Identifier: MPL-2.0

use rp2040_pac::XOSC;

pub unsafe fn init(dev: XOSC) {
    dev.ctrl.write(|w| w.freq_range()._1_15mhz());
    // TODO should this be hardcoded or configurable?
    dev.startup.write(|w| unsafe { w.delay().bits((12_000 + 128) / 256) });
    dev.ctrl.modify(|_, w| w.enable().enable());

    while dev.status.read().enabled().bit_is_clear() {}
}
