# Changelog

## Unreleased - ReleaseDate

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
