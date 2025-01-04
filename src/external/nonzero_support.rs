//! A wrapper type for non-zero UUIDs that provides a more memory-efficient
//! `Option<NonZeroUuid>` representation.

use std::convert::TryFrom;
use std::fmt;
use std::num::NonZeroU128;
use std::ops::Deref;

use crate::Uuid;

/// A UUID that is guaranteed not to be the nil UUID.
///
/// This is useful for representing optional UUIDs more efficiently, as `Option<NonZeroUuid>`
/// takes up the same space as `Uuid`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NonZeroUuid(NonZeroU128);

/// Error returned when attempting to create a `NonZeroUuid` from a nil UUID.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NonZeroUuidError;

impl fmt::Display for NonZeroUuidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attempted to create NonZeroUuid from nil UUID")
    }
}

impl std::error::Error for NonZeroUuidError {}

impl NonZeroUuid {
    /// Creates a non-zero UUID. Returns `None` if the given UUID is the nil UUID.
    #[inline]
    pub fn new(uuid: Uuid) -> Option<Self> {
        let bits = uuid.as_u128();
        NonZeroU128::new(bits).map(Self)
    }

    /// Creates a non-zero UUID without checking if it's the nil UUID.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the UUID is not the nil UUID.
    /// If this constraint is violated, it may lead to undefined behavior when
    /// the resulting NonZeroUuid is used.
    #[inline]
    pub const unsafe fn new_unchecked(uuid: Uuid) -> Self {
        let bits = uuid.as_u128();
        Self(NonZeroU128::new_unchecked(bits))
    }

    /// Returns the underlying `Uuid`.
    #[inline]
    pub fn get(self) -> Uuid {
        Uuid::from_u128(self.0.get())
    }
}

impl TryFrom<Uuid> for NonZeroUuid {
    type Error = NonZeroUuidError;

    fn try_from(uuid: Uuid) -> Result<Self, Self::Error> {
        NonZeroUuid::new(uuid).ok_or(NonZeroUuidError)
    }
}

impl From<NonZeroUuid> for Uuid {
    fn from(nz_uuid: NonZeroUuid) -> Self {
        nz_uuid.get()
    }
}

impl Deref for NonZeroUuid {
    type Target = Uuid;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: We know the bits are valid UUID bits since we only construct
        // NonZeroUuid from valid Uuid values.
        let bits = self.0.get();
        unsafe { &*((&bits as *const u128) as *const Uuid) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonzero_uuid_option_size() {
        assert_eq!(
            std::mem::size_of::<Option<NonZeroUuid>>(),
            std::mem::size_of::<Uuid>()
        );
    }

    #[test]
    fn test_new_with_non_nil() {
        let uuid = Uuid::new_v4();
        let nz_uuid = NonZeroUuid::new(uuid);
        assert!(nz_uuid.is_some());
        assert_eq!(nz_uuid.unwrap().get(), uuid);
    }

    #[test]
    fn test_new_with_nil() {
        let nil_uuid = Uuid::nil();
        let nz_uuid = NonZeroUuid::new(nil_uuid);
        assert!(nz_uuid.is_none());
    }

    #[test]
    fn test_try_from() {
        let uuid = Uuid::new_v4();
        let nz_uuid = NonZeroUuid::try_from(uuid);
        assert!(nz_uuid.is_ok());

        let nil_uuid = Uuid::nil();
        let nz_nil = NonZeroUuid::try_from(nil_uuid);
        assert!(nz_nil.is_err());
    }
}
