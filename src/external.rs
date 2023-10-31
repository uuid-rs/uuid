#[cfg(feature = "arbitrary")]
pub(crate) mod arbitrary_support;
#[cfg(all(uuid_unstable, feature = "borsh"))]
pub(crate) mod borsh_support;
#[cfg(feature = "serde")]
pub(crate) mod serde_support;
#[cfg(feature = "slog")]
pub(crate) mod slog_support;
#[cfg(any(feature = "prost-string", feature = "prost-bytes"))]
pub(crate) mod prost_support;
