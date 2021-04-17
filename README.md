[![Workflow Status](https://github.com/toku-sa-n/xhci/workflows/Rust/badge.svg)](https://github.com/toku-sa-n/xhci/actions?query=workflow%3A%22Rust%22)
[![Crates.io](https://img.shields.io/crates/v/xhci)](https://crates.io/crates/xhci)
![Crates.io](https://img.shields.io/crates/l/xhci)
[![docs.rs](https://docs.rs/xhci/badge.svg)](https://docs.rs/xhci/)

# xhci

A library to handle xHCI.

This crate provides types of the xHCI structures, such as the Registers and Contexts.
Users can use this library to implement a USB device deriver on your own OS.

This crate is `#![no_std]` compatible.

# Examples

```rust
let mut r = unsafe { xhci::Registers::new(MMIO_BASE, mapper) };
let o = &mut r.operational;

o.usbcmd.update(|u| u.set_run_stop(true));
while o.usbsts.read().hc_halted() {}
```
