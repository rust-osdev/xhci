# Changelog

## Unreleased - ReleaseDate
### Added
- Missing error messages of `assert` macros are added.

### Changed
- `set_xxx(true_or_false)` methods of the Registers are split into `set_xxx()` and `clear_xxx()`.

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
