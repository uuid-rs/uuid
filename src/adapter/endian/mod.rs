//! Adapters for handling endianness for [`Uuid`]s.
//!
//! [`Uuid`]: ../../struct.Uuid.html
use crate::prelude::*;

mod core_support;

/// Represents a [`Uuid`] in **big** endian.
///
/// [`Uuid`]: ../../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Big(Bytes);

/// Represents a [`Uuid`] in **little** endian.
///
/// [`Uuid`]: ../../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Little(Bytes);
