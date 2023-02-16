[![Workflow Status](https://github.com/rust-osdev/xhci/workflows/Rust/badge.svg)](https://github.com/rust-osdev/xhci/actions?query=workflow%3A%22Rust%22)
[![Crates.io](https://img.shields.io/crates/v/xhci)](https://crates.io/crates/xhci)
![Crates.io](https://img.shields.io/crates/l/xhci)
[![docs.rs](https://docs.rs/xhci/badge.svg)](https://docs.rs/xhci/)

# xhci

A library to handle xHCI.

This crate provides types of the xHCI structures, such as the Registers and Contexts.
Users can use this library to implement a USB device deriver on your own OS.

This crate is `#![no_std]` compatible.

## Examples

```rust
let mut r = unsafe { xhci::Registers::new(MMIO_BASE, mapper) };
let o = &mut r.operational;

o.usbcmd.update(|u| {
    u.set_run_stop();
});
while o.usbsts.read().hc_halted() {}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
