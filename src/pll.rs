// SPDX-License-Identifier: MPL-2.0

use rp2040_pac::PLL_SYS;

pub unsafe fn init(dev: PLL_SYS, refdiv: u8, fbdiv: u16, post_div1: u8, post_div2: u8) {
    dev.pwr.reset();
    dev.fbdiv_int.reset();
    dev.cs.write(|w| unsafe { w.refdiv().bits(refdiv) });
    dev.fbdiv_int.write(|w| unsafe { w.fbdiv_int().bits(fbdiv) });
    dev.pwr.write(|w| w
        .pd().clear_bit()
        .vcopd().clear_bit()
    );

    while dev.cs.read().lock().bit_is_clear() {}

    dev.prim.write(|w| unsafe { w
        .postdiv1().bits(post_div1)
        .postdiv2().bits(post_div2)
    });
    dev.pwr.modify(|_, w| w.postdivpd().clear_bit());
}
