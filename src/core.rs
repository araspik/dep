//! [`Core`](self): Provides necessary types and functions.
//!
//! See [`Core`] for more information.

/// [`Core`]: Provider of necessary types and functions for [`dep`](crate).
///
/// [`Core`] provides the user-defined functionality on targets.
pub trait Core {
    /// The error type of the installation and removal functions.
    type Error;

    /// Attempts to install the target.
    ///
    /// This function is called at most once, even on failure.
    fn install(&mut self) -> Result<(), Self::Error>;

    /// Attempts to uninstall the target.
    ///
    /// This function is called at most once, even on failure.
    fn remove(&mut self) -> Result<(), Self::Error>;
}
