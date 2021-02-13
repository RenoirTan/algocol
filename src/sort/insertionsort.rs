//! Stores the insertion sort functions.

use std::{
    cmp::{Ord, Ordering},
    convert::AsMut
};
use crate::{
    alreadysorted,
    error::AgcResult,
    utils::priority
};

/// This function sorts a slice using the insertion sort algorithm. In this
/// algorithm, the slice is enumerated from right to left. As it does so, one
/// item is selected in each iteration and is inserted in the correct location
/// in an already sorted sub-slice on the left-hand side of the slice. Once the
/// last item has been enumerated over, this left-hand sub-slice will become
/// the original slice itself. You can choose whether to sort in ascending or
/// descending order by toggling the `ascending` argument between `true` or
/// `false`.
/// 
/// This algorithm's time complexity is O(n^2).
/// In the worst case scenario, (n^2 - n)/2 operations are made.
/// 
/// # Example
/// ```
///     use algocol::sort::insertionsort::insertionsort;
///     let mut array = [5, 4, 3, 2, 1];
///     insertionsort(&mut array[..], true).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn insertionsort<S, T>(
    sequence: &mut S,
    ascending: bool
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    T: Ord
{
    insertionsort_by(sequence, ascending, |a, b| a.cmp(b))
}

/// This function sorts a slice using the insertion sort algorithm. In this
/// algorithm, the slice is enumerated from right to left. As it does so, one
/// item is selected in each iteration and is inserted in the correct location
/// in an already sorted sub-slice on the left-hand side of the slice. Once the
/// last item has been enumerated over, this left-hand sub-slice will become
/// the original slice itself. You can choose whether to sort in ascending or
/// descending order by toggling the `ascending` argument between `true` or
/// `false`. This function requires another function to tell it the order
/// whether 1 element is larger or smaller than the other element.
/// 
/// This algorithm's time complexity is O(n^2).
/// In the worst case scenario, (n^2 - n)/2 operations are made.
/// 
/// # Example
/// ```
///     use algocol::sort::insertionsort::insertionsort_by;
///     let mut array = [5, 4, 3, 2, 1];
///     insertionsort_by(
///         &mut array[..],
///         true,
///         |a, b| a.cmp(b)
///     ).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn insertionsort_by<F, S, T>(
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
    alreadysorted!(result length, return sequence);
    for index in 1..length {
        let mut location = index - 1;
        while if ascending {
            priority::is_gt(
                compare(&sequence[location], &sequence[location+1])
            )
        } else {
            priority::is_lt(
                compare(&sequence[location], &sequence[location+1])
            )
        } {
            sequence.swap(location, location+1);
            if location == 0 {break;}
            location -= 1;
        }
    }
    Ok(sequence)
}