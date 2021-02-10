//! Stores selection sort functions.

use std::{
    cmp::{Ord, Ordering},
    convert::AsMut
};
use crate::{
    alreadysorted,
    error::AgcResult,
    utils::{priority, slice::transfer_element}
};

/// This function sorts a slice using the selection sort algorithm. In each
/// iteration of this algorithm, the next smallest item is taken and
/// transferred to the front of the slice, until all items have been checked
/// and moved if necessary.
/// 
/// This algorithm's time complexity is O(n^2).
/// In the worst case scenario, (n^2 - n)/2 operations are made.
/// 
/// # Example
/// ```
///     use algocol::sort::selectionsort::selectionsort;
///     let mut array = [5, 4, 3, 2, 1];
///     selectionsort(
///         &mut array[..], true
///     ).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn selectionsort<S, T>(
    sequence: &mut S,
    ascending: bool
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    T: Ord
{
    selectionsort_by(sequence, ascending, |a, b| a.cmp(b))
}

/// This function sorts a slice using the selection sort algorithm. In each
/// iteration of this algorithm, the next smallest item is taken and
/// transferred to the front of the slice, until all items have been checked
/// and moved if necessary. A function must be supplied to see whether one
/// object is greater or smaller than the other.
/// 
/// This algorithm's time complexity is O(n^2).
/// In the worst case scenario, (n^2 - n)/2 operations are made.
/// 
/// # Example
/// ```
///     use algocol::sort::selectionsort::selectionsort_by;
///     let mut array = [5, 4, 3, 2, 1];
///     selectionsort_by(
///         &mut array[..], true, |a, b| a.cmp(b)
///     ).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn selectionsort_by<F, T, S>(
    sequence: &mut S,
    ascending: bool,
    compare: F
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    F: Fn(&T, &T) -> Ordering + Copy
{
    let sequence = sequence.as_mut();
    let length = sequence.len();
    alreadysorted!(length, {return Ok(sequence);});
    for subsequence in 0..length {
        let mut extreme: usize = subsequence;
        for (index, element) in sequence.iter().enumerate() {
            if index <= subsequence {
                continue;
            }
            if ascending
            && priority::is_lt(compare(element, &sequence[extreme])) {
                extreme = index;
            } else if !ascending
            && priority::is_gt(compare(element, &sequence[extreme])) {
                extreme = index;
            }
        }
        transfer_element(sequence, extreme, subsequence)?;
    }
    Ok(sequence)
}