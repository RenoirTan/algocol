//! Defines timsort functions.

use std::{
    cmp::{Ord, Ordering, min},
    convert::AsMut,
};
use crate::{
    alreadysorted,
    error::AgcResult,
    sort::{s_insert_if, merge}
};

/// Timsort splits an array into slices of 32 elements (a run) each and sorts
/// each slice with insertion sort before building the array back up with the
/// merge function used by mergesort. This constant represents the default
/// run size used by timsort. You can pass this constant as an argument for
/// `run` in any of the timsort functions.
pub const DEFAULT_RUN: usize = 32;

/// Tim sort is a combination of merge sort and insertion sort. It is meant to
/// work better than merge sort on data that is commonly seen in real-world
/// contexts and was first implemented for sorting lists in Python by Tim
/// Peters. The way timsort works is by splitting the array into individual
/// slices about 32 elements long instead of all the way to 1 element in
/// merge sort. Each 32-element slice is then sorted by insertion sort and
/// built back up by the merge function used in merge sort. The decision to
/// split the array into `run`s of 32 elements makes it faster than splitting
/// the array into slices of 1 element as it avoids 5 more steps of splitting
/// which does not give more time benefits than sorting larger 32-element
/// slices. You can use your own preferred `run` size for this function.
/// However, I recommend that you stick with the default defined by
/// `DEFAULT_RUN`.
/// 
/// # Example
/// ```
///     use algocol::sort::timsort::{timsort, DEFAULT_RUN};
///     let mut array = [5, 4, 3, 2, 1];
///     timsort(&mut array[..], true, DEFAULT_RUN).unwrap();
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn timsort<S, T>(
    sequence: &mut S,
    ascending: bool,
    run: usize
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    T: Ord
{
    timsort_by(sequence, ascending, run, |a, b| a.cmp(b))
}

/// Tim sort is a combination of merge sort and insertion sort. It is meant to
/// work better than merge sort on data that is commonly seen in real-world
/// contexts and was first implemented for sorting lists in Python by Tim
/// Peters. The way timsort works is by splitting the array into individual
/// slices about 32 elements long instead of all the way to 1 element in
/// merge sort. Each 32-element slice is then sorted by insertion sort and
/// built back up by the merge function used in merge sort. The decision to
/// split the array into `run`s of 32 elements makes it faster than splitting
/// the array into slices of 1 element as it avoids 5 more steps of splitting
/// which does not give more time benefits than sorting larger 32-element
/// slices. You can use your own preferred `run` size for this function.
/// However, I recommend that you stick with the default defined by
/// `DEFAULT_RUN`.
/// 
/// This function requires a `compare` function to compare 2 elements with
/// each other.
/// 
/// # Example
/// ```
///     use algocol::sort::timsort::{timsort_by, DEFAULT_RUN};
///     let mut array = (0..100).collect::<Vec<i32>>();
///     array.reverse();
///     timsort_by(
///         &mut array[..], true, DEFAULT_RUN, |a, b| a.cmp(b)
///     ).unwrap();
///     assert_eq!(array, (0..100).collect::<Vec<i32>>());
/// ```
pub fn timsort_by<F, S, T>(
    sequence: &mut S,
    ascending: bool,
    run: usize,
    compare: F
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    F: Fn(&T, &T) -> Ordering + Copy
{
    let sequence = sequence.as_mut();
    let length = sequence.len();
    alreadysorted!(result length, return sequence);
    // If the slice is less than run size, you can use insertion sort on it
    // directly.
    if length <= run {
        return s_insert_if(sequence, ascending, compare);
    }
    for offset in (0..length).step_by(run) {
        s_insert_if(
            &mut sequence[offset..min(offset+run, length)],
            ascending,
            compare
        )?;
    }
    let mut size = run;
    while size < length {
        // The location of the every other odd sub-slice
        // This iterator skips the size of 2 sub-slices to achieve
        // this alternating property
        for left in (0..length).step_by(size*2) {
            // The middle index (see documentation for `merge`)
            // length-1 is constantly checked to prevent indexing
            // errors
            let middle = min(left+size-1, length-1);
            // The last element in the 2 sub-slices.
            let right = min(left+2*size-1, length-1);
            merge(sequence, left, middle, right, ascending, compare)?;
        }
        size <<= 1;
    }
    Ok(sequence)
}