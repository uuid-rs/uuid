use crate::std::fmt;

/// A general error that can occur when working with UUIDs.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error(pub(crate) ErrorKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ErrorKind {
    /// Invalid character in the [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    InvalidCharacter {
        /// The expected characters.
        expected: &'static str,
        /// The invalid character found.
        found: char,
        /// The invalid character position.
        index: usize,
        /// Indicates the [`Uuid`] starts with `urn:uuid:`.
        ///
        /// This is a special case for [`Urn`] adapter parsing.
        ///
        /// [`Uuid`]: ../Uuid.html
        urn: UrnPrefix,
    },
    /// Invalid number of segments in the [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    InvalidGroupCount {
        /// The expected number of segments.
        expected: ExpectedLength,
        /// The number of segments found.
        found: usize,
    },
    /// Invalid length of a segment in a [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    InvalidGroupLength {
        /// The expected length of the segment.
        expected: ExpectedLength,
        /// The length of segment found.
        found: usize,
        /// The segment with invalid length.
        group: usize,
        /// The index of where the group starts
        index: usize,
    },
    /// Invalid length of the [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    InvalidLength {
        /// The expected length(s).
        expected: ExpectedLength,
        /// The invalid length found.
        found: usize,
    },
}

/// The expected length.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum ExpectedLength {
    /// Expected any one of the given values.
    Any(&'static [usize]),
    /// Expected the given value.
    Exact(usize),
}

/// Urn prefix value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum UrnPrefix {
    /// The `urn:uuid:` prefix should optionally provided.
    Optional,
}

impl fmt::Display for ExpectedLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExpectedLength::Any(crits) => write!(f, "one of {:?}", crits),
            ExpectedLength::Exact(crit) => write!(f, "{}", crit),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error(kind)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: ",
            match self.0 {
                ErrorKind::InvalidCharacter { .. } => "invalid character",
                ErrorKind::InvalidGroupCount { .. } =>
                    "invalid number of groups",
                ErrorKind::InvalidGroupLength { .. } => "invalid group length",
                ErrorKind::InvalidLength { .. } => "invalid length",
            }
        )?;

        match self.0 {
            ErrorKind::InvalidCharacter {
                expected,
                found,
                index,
                urn,
            } => {
                let urn_str = match urn {
                    UrnPrefix::Optional => {
                        " an optional prefix of `urn:uuid:` followed by"
                    }
                };

                write!(
                    f,
                    "expected{} {}, found {} at {}",
                    urn_str, expected, found, index
                )
            }
            ErrorKind::InvalidGroupCount {
                ref expected,
                found,
            } => write!(f, "expected {}, found {}", expected, found),
            ErrorKind::InvalidGroupLength {
                ref expected,
                found,
                group,
                ..
            } => write!(
                f,
                "expected {}, found {} in group {}",
                expected, found, group,
            ),
            ErrorKind::InvalidLength {
                ref expected,
                found,
            } => write!(f, "expected {}, found {}", expected, found),
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;
    use crate::std::error;

    impl error::Error for Error {}
}
