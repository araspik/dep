//! [`GenericIdentifier`](self): A generic identifier representing multiple unique ones.
//!
//! See [`GenericIdentifier`] for more information.

use super::UniqueIdentifier;

/// [`GenericIdentifier`]: A generic ID matching multiple unique IDs.
///
/// Generic IDs are always provided by the user.
///
/// Unique identifiers are meant to be convertible to generic identifiers. But because this is only
/// to be done with a specific generic identifier type, the trait is only required from the core.
///
/// In case no generic identifiers exist within whatever system is being implemented, the unique
/// identifier type can be used as a generic identifier type - unique identifiers are also generic
/// identifers (with no priority).
pub trait GenericIdentifier {
    /// A matching unique identifier type.
    type Unique: UniqueIdentifier;

    /// A matching priority type.
    type Priority: Ord;

    /// The matcher function - compares this to a unique identifier and if they match then returns
    /// a priority for that match.
    ///
    /// Higher priorities (ordering defined by [`Ord`]) indicate targets that should be tested
    /// first in dependency map generation.
    fn try_match(&self, other: &Self::Unique) -> Option<Self::Priority>;
}
