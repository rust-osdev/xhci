# Changelog

## Unreleased - ReleaseDate
### Added
- `registers::Doorbell` as an alias of `registers::doorbell::Doorbell`. ([#170])

### Changed
- `registers::Capability` now has read-only accessors instead of read-write ones. ([#167])
- `registers::doorbell::Register` is renamed to `registers::doorbell::Doorbell`. The former still exists, but is deprecated now. ([#167])

### Deprecated
- `registers::doorbell::Register` in favor of `registers::doorbell::Doorbell`. ([#167])

## 0.9.2 - 2023-07-19
### Added
- The sponsor button is now shown on the repository page on GitHub.

### Changed
- CI now uses `giraffate/clippy-action@` instead of `actions-rs/clippy-check` for Clippy check.

### Fixed
- `transfer::Normal`'s data_buffer_pointer calculation is fixed.
- The path to the workflow status is fixed.
- Removed an outdated lint `unaligned_references`.

## 0.9.1 - 2022-09-09
### Added
- `set_0_*` for all RW1C bits, which sets the bit to 0. This prevents the bit from being cleared on write.
- `Default` implementations for registers without RsvdP bits, which allows avoiding a redundant read.

## 0.9.0 - 2022-08-23
### Changed
- The structure of `registers::runtime::InterrupterRegisterSet` is changed, and you can now modify single fields of the Interrupter Register Set via `registers::runtime::Interrupter`. See https://github.com/rust-osdev/xhci/pull/142 and https://github.com/rust-osdev/xhci/pull/142#issuecomment-1222378513 for the background of this change.

### Removed
- `registers::runtime::InterruptRegisterSet` in favor of `registers::runtime::InterrupterRegisterSet`.

## 0.8.7 - 2022-08-17
### Changed
- `registers::runtime::InterruptRegisterSet` is renamed to `registers::runtime::InterrupterRegisterSet` (note: Interrupt**er**). The former still exists but is an alias of the latter and is deprecated now.

### Deprecated
- `registers::runtime::InterruptRegisterSet` in favor of `registers::runtime::InterrupterRegisterSet`.

## 0.8.6 - 2022-07-31
### Fixed
- The wrong bit range in the implementation of `TryFrom<[u32; 4]>` for TRB structs is fixed.

## 0.8.5 - 2022-07-21
### Added
- Added the implementation of `TryFrom<[u32; 4]>` to TRB structs in `ring::trb::command` and `ring::trb::transfer`

### Changed
- Clippy's function size limitation setting is removed.

## 0.8.4 - 2022-05-29
### Fixed
- Wrong bit range in `StructuralParameters2::max_scratchpad_buffers` is fixed.

## 0.8.3 - 2022-04-20
### Added
- License and contribution notes are added to the README.
- Dependabot is enabled.

### Changed
- Switched to Rust edition 2021.

### Fixed
- Outdated code example in `README.md` is fixed.
- Removed all the use of deprecated code.
- Wrong masking in `EventRingDequeuePointerRegister::event_ring_dequeue_pointer` is fixed.

## 0.8.2 - 2021-05-13
### Removed
- `#![deny(warnings)]` is removed from the source code.

## 0.8.1 - 2021-05-12
### Fixed
- The members of `XhciLocalMemory` are now public.

## 0.8.0 - 2021-05-12
### Changed
- The most part of `trb` module is rewritten.
    - `set_xxx(true_or_false)` methods are split into `set_xxx()` and `clear_xxx()`.
    - Bit setters, bit clearers, and field setters now return mutable references to `Self`
    - `transfer::SetupStage::set_trb_transfer_length` is removed.
- Missing members and accessors of TRBs are added.

## 0.7.0 - 2021-05-12
### Added
- Missing error messages of `assert` macros are added.

### Changed
- `set_xxx(true_or_false)` methods of the Registers and the Extended Capabilities are split into `set_xxx()` and `clear_xxx()`.
- `CapabilityParameters1::max_primary_stream_array_size` is renamed to `CapabilityParameters1::maximum_primary_stream_array_size`.
- Bit setter and clearer now return mutable references to `Self`.
- `extended_capabilities::HciExtendedPowerManagement` is rewritten so that it contains members.
- `context` module is rewritten.

## 0.6.0 - 2021-04-27
### Added
- A note about `Mapper` is added to the documentation of `Registers`
- Missing members and accessors of the Extended Capabilities are added.

### Changed
- `USBLegacySupportCapability` is redefined as `USBLegacySupport`. Now it includes the accessor to the USB Legacy Support Control and Status Register.

### Fixed
- Syntax highlight is applied to the code example in README.

## 0.5.6 - 2021-04-16
### Added
- Missing members and accessors of the Operational Registers and the Runtime Registers are added.

## 0.5.5 - 2021-04-15
### Added
- The `doorbell_stream_id` and `set_doorbell_stream_id` methods are added to `doorbell::Register`.

## 0.5.4 - 2021-04-15
### Added
- Missing members of `Capability` are added.

## 0.5.3 - 2021-04-12
### Fixed
- The typo of the URL to the repository is fixed.

## 0.5.2 - 2021-02-25
### Added
- `From` trait is implemented for the `Allowed` enum to convert from the structs of TRBs to the enum.

## 0.5.1 - 2021-02-15
### Added
- The missing Completion Codes are added to the `CompletionCode`.

### Changed
- `CompletionCode` becomes an exhausive enum.

## 0.5.0 - 2021-02-15
### Added
- All the missing getters of the Transfer TRBs are implemented.

### Changed
- The debug prints of the all TRBs now print the value of each field.

### Fixed
- `ConfigureEndpoint::deconfigure` and `SetTrDequeuePointer::slot_id` wrongly took the mutable references to the `self`. They now take the immutable references.

## 0.4.1 - 2021-02-05
### Added
- All types now implement the `Debug` trait.
- Code examples are added.
- `ring` module is added.

### Fixed
- Wrong codes in documentations are fixed.

## 0.4.0 - 2021-01-31
### Changed
- Methods of the `register` module now panic if it fails to create an accessor.

## 0.3.0 - 2021-01-31
### Added
- The `context` module.

### Changed
- Methods now panic if an error occurs.

### Removed
- `error::Error`

### Fixed
- Add a missing document.

## 0.2.6 - 2021-01-28
### Added
- Reexport `error::Error` as `Error`.
- Define accessors to xHCI registers and xHCI Extended Capabilities.

## 0.2.5 - 2021-01-23
### Added
- Implement `Copy` and `Clone` for register types.

## 0.2.4 - 2021-01-22
### Added
- `extended_capabilities::usb_legacy_support_capability::UsbLegacySupportCapability`.

## 0.2.3 - 2021-01-22
### Fixed
- `registers::operational::UsbCommandRegister::set_host_controller_reset` updated the wrong bit.

## 0.2.2 - 2021-01-22
### Added
- Implement `Debug` for `error::Error`.

## 0.2.1 - 2021-01-22
### Added
- Implement `Debug` for the register types.

## 0.2.0 - 2021-01-22
### Changed
- Rename `EventRingDequeuePointerRegister::set` to `EventRingDequeuePointerRegister::set_event_ring_dequeue_pointer`.

## 0.1.0 - 2021-01-22
### Added
- Initial version.

[#170]: https://github.com/rust-osdev/xhci/pull/170
[#167]: https://github.com/rust-osdev/xhci/pull/167
