//! A collection of sorting algorithms.
//! 
//! All of the functions defined in this module and its child modules take in
//! a mutable reference which can be converted to `&mut [T]` via `AsMut<[T]>`.
//! `T` must implement `std::cmp::Ord` so that the functions in this module
//! are able to determine whether object A goes before object B or not. In
//! addition, you must guarantee that the data type that you are trying to sort
//! does not have to have a stable memory address, as these sorting functions
//! will be moving objects around to achieve order. To learn more about this,
//! please read the module-level documentation for `std::pin`.
//! 
//! In order to reduce space complexity to O(1), items are sorted one-by-one
//! in the given slice itself (not in another slice). Hence, if an error
//! happens midway, it means that the elements in the slice would not be in the
//! same order as before.
//! 
//! # Available Algorithms
//! 1. Bubble Sort
//! 2. Selection Sort
//! 3. Insertion Sort
//! 4. Merge Sort
//! 
//! # Notes
//! 
//! In this module, you will commonly see the following snippet of code:
//! 
//! ```ignore
//!     let sequence: &mut [T] = sequence.as_mut();
//!     let length: usize = sequence.len();
//!     if length <= 1 {
//!         return Ok(sequence);
//!     }
//! ```
//! 
//! The first line is simply coercing the mutable slice out of your input.
//! The if block below that checks to see if the length is 1 or less and
//! returns `Ok` if so. This shortcut can be done as a slice is already sorted
//! if only there is nothing or there is only one thing in the slice.

use std::{
    cmp::{Ord, Ordering},
    convert::AsRef
};
use crate::utils::priority;

pub mod bubblesort;
pub mod insertionsort;
pub mod mergesort;
pub mod quicksort;
pub mod selectionsort;
pub mod timsort;

pub use crate::sort::{
    bubblesort::*,
    insertionsort::*,
    mergesort::*,
    quicksort::*,
    selectionsort::*,
    timsort::*
};

pub use self::{
    bubblesort::{
        bubblesort as s_bubble_i,
        bubblesort_by as s_bubble_if,
    },
    insertionsort::{
        insertionsort as s_insert_i,
        insertionsort_by as s_insert_if
    },
    mergesort::{
        merge,
        mergesort as s_merge_i,
        mergesort_by as s_merge_if,
        mergesort_recursively as s_merge_r,
        mergesort_recursively_by as s_merge_rf
    },
    quicksort::{
        partition
    },
    selectionsort::{
        selectionsort as s_select_i,
        selectionsort_by as s_select_if
    },
    timsort::{
        timsort as s_tim_i,
        timsort_by as s_tim_if
    }
};

/// Checks to see if a slice is correctly ordered in ascending or descending
/// order. The sequence that you passed must have elements that implement
/// `std::cmp::Ord`. If you want to check if the sequence is in ascending
/// order, the second argument that you pass in should be set to `true`, else
/// if you want to check if it is in descending order, set the second argument
/// to `false`. If the sequence is sorted in your desired order, `true` is
/// returned.
/// 
/// # Example
/// ```
///     use algocol::sort::is_sorted;
///     let array = [5, 4, 3, 2, 1];
///     assert!(is_sorted(&array[..], false));
/// ```
/// 
/// This function immediately returns `true` if the length of `sequence` is 0
/// or 1.
pub fn is_sorted<S, T>(sequence: &S, ascending: bool) -> bool
where
    S: AsRef<[T]> + ?Sized,
    T: Ord
{
    is_sorted_by(sequence, ascending, |a, b| a.cmp(b))
}

/// Checks to see if a slice is correctly ordered in ascending or descending
/// order. If you want to check if the sequence is in ascending
/// order, the second argument that you pass in should be set to `true`, else
/// if you want to check if it is in descending order, set the second argument
/// to `false`. If the sequence is sorted in your desired order, `true` is
/// returned. `compare` is a function or closure that you must pass in to this
/// function to let it know the diffence in order between 2 objects in your
/// sequence.
/// 
/// # Example
/// ```
///     use algocol::sort::is_sorted_by;
///     let array = [5, 4, 3, 2, 1];
///     assert!(is_sorted_by(&array[..], false, |a, b| a.cmp(b)));
/// ```
/// 
/// This function immediately returns `true` if the length of `sequence` is 0
/// or 1.
pub fn is_sorted_by<F, S, T>(
    sequence: &S,
    ascending: bool,
    compare: F
) -> bool
where
    S: AsRef<[T]> + ?Sized,
    F: Fn(&T, &T) -> Ordering + Copy
{
    let sequence = sequence.as_ref();
    let length = sequence.len();
    if length <= 1 {
        return true;
    }
    if ascending {
        for index in 0..length-1 {
            if priority::is_gt(compare(&sequence[index], &sequence[index+1])) {
                return false;
            }
        }
    } else {
        for index in 0..length-1 {
            if priority::is_lt(compare(&sequence[index], &sequence[index+1])) {
                return false;
            }
        }
    }
    true
}