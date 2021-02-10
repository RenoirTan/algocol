//! # Algocol
//! 
//! *An **algo**rithm **col**lection.*
//! 
//! `algocol` is a collection of algorithms. I do not intend for this crate to
//! be used in actual developmental projects but more in educational settings
//! where you can inspect each function by pulling out an IDE to see how each
//! algorithm works.
//! 
//! By default, the algorithms in this crate uses iterative methods over
//! recursive methods. If recursive functions are available, there will be a
//! `recursive(ly)` in their name, and their method of recursion is top-down.
//! 
//! Some functions in this crate have been re-exported with shortened names
//! with useful classification affixes. For example,
//! `algocol::sort::mergesort_recursively_by` is re-exported as
//! `s_merge_rf`. The `s` prefix means that this function sorts a sequence
//! such as a slice. `merge` is the algorithm that the function uses to sort
//! the slice. `rf` is actually a compound suffix. `r` means that the function
//! is recursive and `f` means that a function must be provided as an
//! argument for a task used by our mergesort function, in this case, the
//! input function is called `compare` and returns the `std::cmp::Ordering`
//! between 2 elements.
//! 
//! Below are the prefixes currently used by this crate:
//! 1. `s`: For functions which sort slices
//! 2. `sc`: Functions which search for an element in a sequence
//! 3. `sl`: Utility functions on slices
//! 
//! The following suffix parts are used in this crate:
//! 1. `i`: This function is iterative (as opposed to recursive)
//! 2. `r`: This function is recursive
//! 3. `f`: This function requires an auxiliary function

pub mod binarysearch;
pub mod error;
pub mod macros;
pub mod sort;
pub mod utils;

pub use crate::error::{AgcError, AgcErrorKind, AgcResult};