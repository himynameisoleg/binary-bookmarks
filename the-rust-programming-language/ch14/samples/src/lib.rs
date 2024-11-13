// Documenting lib code.
// Run with `cargo doc --open`

//! # Sample Crate
//!
//! `samples` is a collection of utilities to make ...
//! this is a style of comment that describes the contained item,
//! rather than the thing just below the comment.

/// Adds one to number given.
///
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = samples::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
