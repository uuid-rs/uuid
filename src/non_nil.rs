//! A wrapper type for nil UUIDs that provides a more memory-efficient
//! `Option<NonNilUuid>` representation.

use core::convert::TryFrom;
use std::{fmt, num::NonZeroU128};

use crate::Uuid;

/// A UUID that is guaranteed not to be the nil UUID.
///
/// This is useful for representing optional UUIDs more efficiently, as `Option<NonNilUuid>`
/// takes up the same space as `Uuid`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NonNilUuid(NonZeroU128);

impl fmt::Display for NonNilUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Uuid::from(*self))
    }
}

impl NonNilUuid {
    /// Returns the underlying `Uuid`.
    #[inline]
    pub const fn get(self) -> Uuid {
        Uuid::from_u128(self.0.get())
    }
}

impl From<NonNilUuid> for Uuid {
    /// Converts a [`NonNilUuid`] back into a [`Uuid`].
    ///
    /// # Examples
    /// ```
    /// use uuid::{non_nil::NonNilUuid, Uuid};
    /// use std::convert::TryFrom;
    ///
    /// let uuid = Uuid::from_u128(0x0123456789abcdef0123456789abcdef);
    /// let non_nil = NonNilUuid::try_from(uuid).unwrap();
    /// let uuid_again = Uuid::from(non_nil);
    ///
    /// assert_eq!(uuid, uuid_again);
    /// ```
    fn from(non_nil: NonNilUuid) -> Self {
        Uuid::from_u128(non_nil.0.get())
    }
}

impl TryFrom<Uuid> for NonNilUuid {
    type Error = &'static str;

    /// Attempts to convert a [`Uuid`] into a [`NonNilUuid`].
    ///
    /// # Examples
    /// ```
    /// use uuid::{non_nil::NonNilUuid, Uuid};
    /// use std::convert::TryFrom;
    ///
    /// let uuid = Uuid::from_u128(0x0123456789abcdef0123456789abcdef);
    /// let non_nil = NonNilUuid::try_from(uuid).unwrap();
    /// ```
    fn try_from(uuid: Uuid) -> Result<Self, Self::Error> {
        NonZeroU128::new(uuid.as_u128())
            .map(Self)
            .ok_or("Attempted to convert nil Uuid to NonNilUuid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_nil_with_option_size() {
        assert_eq!(
            std::mem::size_of::<Option<NonNilUuid>>(),
            std::mem::size_of::<Uuid>()
        );
    }

    #[test]
    fn test_non_nil() {
        let uuid = Uuid::from_u128(0x0123456789abcdef0123456789abcdef);
        let nn_uuid = NonNilUuid::try_from(uuid);

        assert!(nn_uuid.is_ok());
        assert_eq!(Uuid::from(nn_uuid.unwrap()), uuid);

        let nil_uuid = Uuid::nil();
        let nn_uuid = NonNilUuid::try_from(nil_uuid);
        assert!(nn_uuid.is_err());
    }
}
