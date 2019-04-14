//! [`UniqueIdentifier`](self): Unique identification for a single target.
//!
//! See [`UniqueIdentifier`] for more information.

use super::GenericIdentifier;
use std::hash::Hash;

/// [`UniqueIdentifier`]: A unique ID for a single target.
///
/// This identifier should be stored by the target it identifies.
///
/// Unique identifiers are meant to be convertible to generic identifiers. But because this is only
/// to be done with a specific generic identifier type, the trait is only required from the core.
///
/// In case no generic identifiers exist within whatever system is being implemented, the unique
/// identifier type can be used as a generic identifier type - unique identifiers are also generic
/// identifers (with no priority).
pub trait UniqueIdentifier: Eq + Hash {}

impl<T: UniqueIdentifier> GenericIdentifier for T {
    /// The unique identifier type is this one itself.
    type Unique = T;

    /// The priority type is nothing, i.e an empty tuple.
    type Priority = ();

    /// The matcher function simply tests equality.
    fn try_match(&self, other: &Self::Unique) -> Option<Self::Priority> {
        if self == other {
            Some(())
        } else {
            None
        }
    }
}
