//! Adapters for various formats for [`Uuid`]s
//!
//! [`Uuid`]: ../struct.Uuid.html

/// The length of a Hyphenated [`Uuid`] string.
///
/// [`Uuid`]: ../struct.Uuid.html
pub const UUID_HYPHENATED_LENGTH: usize = 36;

/// The length of a Simple [`Uuid`] string.
///
/// [`Uuid`]: ../struct.Uuid.html
pub const UUID_SIMPLE_LENGTH: usize = 32;

/// The length of a Urn [`Uuid`] string.
///
/// [`Uuid`]: ../struct.Uuid.html
pub const UUID_URN_LENGTH: usize = 45;
