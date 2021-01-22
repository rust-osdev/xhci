//! A module containing things representing errors.

/// An enum representing errors.
pub enum Error {
    /// The passed address is not aligned correctly.
    NotAligned {
        /// The address must be `alignment` byte aligned.
        alignment: u64,
        /// The address passed as an argument.
        address: u64,
    },
}
