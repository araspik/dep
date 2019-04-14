//! [`Identifier`](self): Identification for targets.
//!
//! Identifiers come in two variants:
//! * Unique identifiers: Only match a single target, and is stored by that target. It should be
//!   usable within a `HashMap`.
//! * Generic identifiers: Matches multiple unique identifiers, and is provided for by the user. It
//!   does not even require equality matching.
//!
//! Both are traits to be implemented by the user.
//!
//! Note that unique identifiers do not have to be `Clone`-able, and should be handled using `Rc`.
//! This may be changed whenever specialization comes along.
//!
//! Also: seeing that unique identifiers form a subset of generic identifiers, one should be able
//! to convert unique identifiers to generic ones. This allows installation, removal, and matching
//! code to accept generic identifiers, but be usable with unique identifiers too. This could be
//! enhanced with specialization.

pub mod generic;
pub mod unique;

pub use self::generic::GenericIdentifier;
pub use self::unique::UniqueIdentifier;
