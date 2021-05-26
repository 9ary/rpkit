// SPDX-License-Identifier: MPL-2.0

#![no_std]

#![deny(unsafe_op_in_unsafe_fn)]

pub use rp2040_pac as pac;

#[cfg(feature = "rt")]
pub use pac::interrupt;

pub mod clocks;
pub mod pll;
pub mod xosc;
