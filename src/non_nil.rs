//! A wrapper type for nil UUIDs that provides a more memory-efficient
//! `Option<NonNilUuid>` representation.

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

impl From<NonNilUuid> for Uuid {
    /// Converts a [`NonNilUuid`] back into a [`Uuid`].
    ///
    /// # Examples
    /// ```
    /// use uuid::{non_nil::NonNilUuid, Uuid};
    ///
    /// let uuid = Uuid::new_v4();
    /// let non_nil = NonNilUuid::from(uuid);
    /// let uuid_again = Uuid::from(non_nil);
    ///
    /// assert_eq!(uuid, uuid_again);
    /// ```
    fn from(non_nil: NonNilUuid) -> Self {
        Uuid::from_u128(non_nil.0.get())
    }
}

impl From<Uuid> for NonNilUuid {
    /// Converts a [`Uuid`] into a [`NonNilUuid`].
    ///
    /// # Panics
    /// Panics if the input UUID is nil (all zeros).
    ///
    /// # Examples
    /// ```
    /// use uuid::{non_nil::NonNilUuid, Uuid};
    ///
    /// let uuid = Uuid::new_v4();
    /// let non_nil = NonNilUuid::from(uuid);
    /// ```
    fn from(uuid: Uuid) -> Self {
        NonZeroU128::new(uuid.as_u128())
            .map(Self)
            .expect("Attempted to convert nil Uuid to NonNilUuid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonzero_uuid_option_size() {
        assert_eq!(
            std::mem::size_of::<Option<NonNilUuid>>(),
            std::mem::size_of::<Uuid>()
        );
    }

    #[test]
    fn test_new_with_non_nil() {
        let uuid = Uuid::new_v4();
        let nn_uuid = NonNilUuid::from(uuid);
        assert_eq!(Uuid::from(nn_uuid), uuid);
    }

    #[test]
    #[should_panic(expected = "Attempted to convert nil Uuid to NonNilUuid")]
    fn test_new_with_nil() {
        let nil_uuid = Uuid::nil();
        let _ = NonNilUuid::from(nil_uuid);
    }
}
