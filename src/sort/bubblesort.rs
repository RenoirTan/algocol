//! Stores the bubble sort functions.

use std::{
    convert::AsMut,
    cmp::{Ord, Ordering}
};
use crate::{
    error::AgcResult,
    utils::priority
};

/// This function sorts a slice using the bubblesort algorithm, where each
/// 2 adjacent elements in the sequence are checked that they are in order and
/// swapped if not, until all elements have been verified to be in their
/// correct order. You can choose whether to sort in ascending or descending
/// order by toggling the `ascending` argument between `true` or `false`.
/// 
/// This algorithm's time complexity is O(n^2).
/// In the worst case scenario, (n^2 - n)/2 operations are made.
/// 
/// # Example
/// ```
///     use algocol::sort::bubblesort::bubblesort;
///     let mut array = [5, 4, 3, 2, 1];
///     bubblesort(&mut array[..], true).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn bubblesort<S, T>(
    sequence: &mut S, ascending: bool
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    T: Ord
{
    bubblesort_by(sequence, ascending, |a, b| a.cmp(b))
}

/// This function sorts a slice using the bubblesort algorithm, where each
/// 2 adjacent elements in the sequence are checked that they are in order and
/// swapped if not, until all elements have been verified to be in their
/// correct order. You can choose whether to sort in ascending or descending
/// order by toggling the `ascending` argument between `true` or `false`. This
/// function requires another function to tell it the order whether 1 element
/// is larger or smaller than the other element.
/// 
/// This algorithm's time complexity is O(n^2).
/// In the worst case scenario, (n^2 - n)/2 operations are made.
/// 
/// # Example
/// ```
///     use algocol::sort::bubblesort::bubblesort_by;
///     let mut array = [5, 4, 3, 2, 1];
///     bubblesort_by(
///         &mut array[..], true, |a, b| a.cmp(b)
///     ).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn bubblesort_by<F, S, T>(
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
    if length <= 1 {
        return Ok(sequence);
    }
    let mut sorted = false;
    if ascending {
        while !sorted {
            sorted = true;
            for index in 1..length {
                if priority::is_gt(
                    compare(&sequence[index-1], &sequence[index])
                ) {
                    sequence.swap(index, index-1);
                    sorted = false;
                }
            }
        }
    } else {
        while !sorted {
            sorted = true;
            for index in 1..length {
                if priority::is_lt(
                    compare(&sequence[index-1], &sequence[index])
                ) {
                    sequence.swap(index, index-1);
                    sorted = false;
                }
            }
        }
    }
    Ok(sequence)
}