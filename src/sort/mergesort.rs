//! Defines mergesort functions and merge function used by mergesort

use std::{
    cmp::{Ord, Ordering, min},
    convert::AsMut
};
use crate::{
    alreadysorted,
    error::{AgcResult, AgcError, AgcErrorKind},
    utils::{priority, slice::transfer_element}
};

#[warn(deprecated_in_future)]
/// **This function is only meant to be used by the functions in this crate.
/// However, it has been set to public to allow doctests to be run. In future
/// releases, this function may become private.**
/// 
/// This is the merge algorithm used by merge sort. This function takes a
/// contiguous segment of a slice, and merges the 2 parts of the slices into
/// one ordered slice. It assumes that the 2 sub-slices are already sorted in
/// the correct order, so when it merges the 2 slices together, the final slice
/// will be ordered correctly. The location and sizes of the 2 slices must be
/// provided by filling in the parameters for `left`, `middle` and `right`.
/// `left` tells the function where the first element of the first sub-slice
/// is, `middle` is the location of the last element of the first sub-slice and
/// `right` is the location of the last element of the second sub-slice. This
/// means that the first element of the second sub-slice will be `middle+1`,
/// assuming that `right > middle`. If `middle == right`, the length of the
/// second sub-slice is 0. The value of `left`, `middle` and `right` must be
/// in the following order: `left <= middle <= right`.
/// 
/// `compare` is the function used to check the ordering of 2 elements.
/// 
/// # Notes
/// 
/// This function merges a slice in-place.
/// 
/// # Example
/// 
/// ```
///     use algocol::sort::mergesort::merge;
///     let mut array = [7, 6, 1, 3, 5, 2, 4, 6, 8];
///     let result = merge(&mut array[..], 2, 4, 8, true, |a, b| a.cmp(b));
///     println!("{:?}", result);
///     assert_eq!(array, [7, 6, 1, 2, 3, 4, 5, 6, 8]);
/// ```
pub fn merge<'t, F, T>(
    slice: &'t mut [T],
    left: usize,
    middle: usize,
    right: usize,
    ascending: bool,
    compare: F
) -> AgcResult<&'t mut [T]>
where
    F: Fn(&T, &T) -> Ordering + Copy
{
    // Start of error checking section
    if left > middle {
        return Err(
            AgcError::new(
                AgcErrorKind::WrongOrder,
                format!(
                    "Left ({}) cannot be greater than middle ({})",
                    left,
                    middle
                )
            )
        );
    } else if middle > right {
        return Err(
            AgcError::new(
                AgcErrorKind::WrongOrder,
                format!(
                    "Right ({}) cannot be smaller than middle ({})",
                    right,
                    middle
                )
            )
        );
    }
    let length = slice.len();
    if left > length {
        return Err(AgcError::new(AgcErrorKind::OutOfBounds, format!(
            "Left ({}) is out of bounds.",
            left
        )));
    } else if middle > length {
        return Err(AgcError::new(AgcErrorKind::OutOfBounds, format!(
            "Middle ({}) is out of bounds.",
            middle
        )));
    } else if right > length {
        return Err(AgcError::new(AgcErrorKind::OutOfBounds, format!(
            "Right ({}) is out of bounds.",
            right
        )));
    }
    // End of error checking section
    // [deposit..., left..., right...];
    // ^ d          ^ l      ^ r
    // deposit_size left_size right_size
    let mut left_size = middle - left + 1;
    let mut right_size = right - middle;
    let mut deposit_size = 0;
    while left_size > 0 && right_size > 0 {
        if priority::is_lt(
            compare(
                &slice[left+deposit_size],
                &slice[left+deposit_size+left_size]
            )
        ) == ascending {
            left_size -= 1;
        } else {
            transfer_element(
                slice,
                left+deposit_size+left_size,
                left+deposit_size
            )?;
            right_size -= 1;
        }
        deposit_size += 1;
    }
    Ok(slice)
}

/// This function sorts an unordered slice using the merge sort algorithm.
/// This function works by splitting the sequence into smaller slices and
/// sorting them one by one, before working its way up by **merging** the
/// smaller slices which have already been sorted.
/// 
/// This algorithm's time complexity is O(n^2).
/// 
/// # Example
/// ```
///     use algocol::sort::mergesort::mergesort;
///     let mut array = [5, 4, 3, 2, 1];
///     mergesort(&mut array[..], true).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn mergesort<S, T>(
    sequence: &mut S,
    ascending: bool
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    T: Ord
{
    mergesort_by(sequence, ascending, |a, b| a.cmp(b))
}

/// Iterative merge sort with a compare functions which determines the order
/// of 2 elements in the sequence. This function works by splitting the
/// sequence into smaller slices and sorting them one by one, before working
/// its way up by **merging** the smaller slices which have already been
/// sorted.
/// 
/// This algorithm's time complexity is O(n^2). This function is adapted from
/// GeeksforGeeks' C++
/// [implemetation](https://www.geeksforgeeks.org/iterative-merge-sort/).
/// 
/// # Example
/// ```
///     use algocol::sort::mergesort::mergesort_by;
///     let mut array = [5, 4, 3, 2, 1];
///     mergesort_by(
///         &mut array[..], true, |a, b| a.cmp(b)
///     ).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn mergesort_by<F, S, T>(
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
    let mut size: usize = 1;
    // Size of each sub-slice
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
            merge(sequence, left, middle, right, ascending, &compare)?;
        }
        size *= 2;
    }
    Ok(sequence)
}

/// This function sorts an unordered slice using the merge sort algorithm.
/// This function works by splitting the sequence into smaller slices
/// recursively and sorting them one by one, before working its way up by
/// **merging** the smaller slices which have already been sorted.
/// 
/// This algorithm's time complexity is O(n^2).
/// 
/// # Example
/// ```
///     use algocol::sort::mergesort::mergesort_recursively;
///     let mut array = [5, 4, 3, 2, 1];
///     mergesort_recursively(
///         &mut array[..], true
///     ).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn mergesort_recursively<S, T>(
    sequence: &mut S,
    ascending: bool
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    T: Ord
{
    mergesort_recursively_by(sequence, ascending, |a: &T, b: &T| a.cmp(b))
}

/// Iterative merge sort with a compare functions which determines the order
/// of 2 elements in the sequence. This function works by splitting the
/// sequence into smaller slices recursively and sorting them one by one,
/// before working its way up by **merging** the smaller slices which have
/// already been sorted.
/// 
/// This algorithm's time complexity is O(n^2).
/// 
/// # Example
/// ```
///     use algocol::sort::mergesort::mergesort_recursively_by;
///     let mut array = [5, 4, 3, 2, 1];
///     mergesort_recursively_by(
///         &mut array[..], true, |a, b| a.cmp(b)
///     ).unwrap(); // 10 operations are made.
///     assert_eq!(array, [1, 2, 3, 4, 5]);
/// ```
pub fn mergesort_recursively_by<'t, F, S, T>(
    sequence: &'t mut S,
    ascending: bool,
    compare: F
) -> AgcResult<&'t mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    F: Fn(&T, &T) -> Ordering + Copy
{
    let sequence = sequence.as_mut();
    let length = sequence.len();
    if length <= 1 {
        return Ok(sequence);
    }
    let middle = length/2;
    mergesort_recursively_by(&mut sequence[..middle], ascending, compare)?;
    mergesort_recursively_by(&mut sequence[middle..], ascending, compare)?;
    merge(sequence, 0, middle-1, length-1, ascending, compare)?;
    Ok(sequence)
}