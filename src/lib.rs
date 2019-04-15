//! # [`dep`](crate): A dependency management library
//!
//! This is an attempt to simplify the job of every single application or library
//! that does its own dependency management. No matter whether it is for packages,
//! services, extensions, modules, _whatever_ - this library aims to do that job for
//! it.
//!
//! ### How?
//! All common (and some uncommon) features have been implemented (hopefully!) over
//! the generic concept of a _target_ - a single entity with respect to dependency
//! management.
//!
//! ### Features
//! Some features (to be) implemented are:
//! * Features ("flags" for modifying the installation and building process of the
//!   target)
//! * Group identifiers (IDs matching multiple targets, e.g to allow choosing one of
//!   multiple dependencies)
//! * Build-time vs. run-time dependencies (a chain of run-time dependencies are
//!   allowed to have cyclic dependencies)
//! * Conflicts
//! * Safe failure in installation or removal (installations and removals are
//!   ordered in such a way that a failure does not break anything)
//!
//! ### Missing something
//! If some feature that could be considered generally relevant is missing, then
//! please go to the issues page! Feature requests are always welcome and will be
//! discussed thoroughly, and then hopefully added to `dep`.

pub mod core;
pub mod prelude;
