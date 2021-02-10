//! Stores definitions of macros used by this crate.

/// Check if a slice is already sorted by checking if the length of the slice
/// less than or equal to 1. If so, it runs an expression.
#[macro_export]
macro_rules! alreadysorted {
    () => {
        return Ok(());
    };
    ($length: expr) => {
        if $length <= 1 {
            return Ok(());
        }
    };
    ($length: expr, $then: expr) => {
        if $length <= 1 {$then}
    }
}