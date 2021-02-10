//! Binary search functions

use crate::{
    error::{AgcError, AgcErrorKind, AgcResult},
    utils::priority,
    sort::{is_sorted, is_sorted_by}
};
use std::{
    cmp::{Ord, Ordering},
    convert::AsRef
};

pub use binarysearch_unchecked as sc_binary_ui;
pub use binarysearch_unchecked_by as sc_binary_uif;
pub use binarysearch as sc_binary_i;
pub use binarysearch_by as sc_binary_if;

/// Find where an `item` should be in an ordered `sequence`. This function
/// does not check to see if the sequence has been ordered properly or not,
/// hence the "unchecked" suffix at the end. If the `item` is not found in the
/// `sequence`, the location returned is where it should be in the sequence.
/// If `item` is greater than the last element in the `sequence`,
/// `sequence.as_ref().len()` is returned. If multiple elements with the same
/// priority exist, then one of them is chosen as the result.
/// 
/// # Examples
/// 
/// This is the result for a correctly ordered array.
/// 
/// ```
///     use algocol::binarysearch::binarysearch_unchecked;
///     let array = [0, 2, 4, 6, 8];
///     let location = binarysearch_unchecked(&array[..], &5, true);
///     assert_eq!(location, 3); // If 5 were to be inserted into `array`
///                              // while making sure that the array remains
///                              // sorted, 5 should be placed at index 3.
/// ```
/// 
/// However, this is what happens if the array is not sorted,
/// 
/// ```
///     use algocol::binarysearch::binarysearch_unchecked;
///     let array = [0, 8, 2, 6, 4];
///     let location = binarysearch_unchecked(&array[..], &5, true);
///     assert_eq!(location, 5);
/// ```
/// 
/// Index 5 is returned as 5 is greater than 4.
pub fn binarysearch_unchecked<S, T>(
    sequence: &S,
    item: &T,
    ascending: bool
) -> usize
where
    S: AsRef<[T]> + ?Sized,
    T: Ord
{
    binarysearch_unchecked_by(sequence, item, ascending, |a, b| a.cmp(b))
}

/// Find where an `item` should be in an ordered `sequence`. This function
/// does not check to see if the sequence has been ordered properly or not,
/// hence the "unchecked" suffix at the end. If the `item` is not found in the
/// `sequence`, the location returned is where it should be in the sequence.
/// If `item` is greater than the last element in the `sequence`,
/// `sequence.as_ref().len()` is returned. If multiple elements with the same
/// priority exist, then one of them is chosen as the result. A function that
/// can compare the level of priority between 2 `T`s must be provided.
/// 
/// # Examples
/// 
/// This is the result for a correctly ordered array.
/// 
/// ```
///     use algocol::binarysearch::binarysearch_unchecked_by;
///     let array = [0, 2, 4, 6, 8];
///     let location = binarysearch_unchecked_by(
///         &array[..],
///         &5,
///         true,
///         |a, b| a.cmp(b)
///     );
///     assert_eq!(location, 3); // If 5 were to be inserted into `array`
///                              // while making sure that the array remains
///                              // sorted, 5 should be placed at index 3.
/// ```
/// 
/// However, this is what happens if the array is not sorted,
/// 
/// ```
///     use algocol::binarysearch::binarysearch_unchecked;
///     let array = [0, 8, 2, 6, 4];
///     let location = binarysearch_unchecked(&array[..], &5, true);
///     assert_eq!(location, 5);
/// ```
/// 
/// Index 5 is returned as 5 is greater than 4.
pub fn binarysearch_unchecked_by<F, S, T>(
    sequence: &S,
    item: &T,
    ascending: bool,
    compare: F
) -> usize
where
    S: AsRef<[T]> + ?Sized,
    F: Fn(&T, &T) -> Ordering + Copy
{
    let sequence = sequence.as_ref();
    let length = sequence.len();
    if length == 0 {
        return 0;
    } else if length == 1 {
        let ordering = compare(item, &sequence[0]);
        return if ascending {
            if priority::is_le(ordering) {
                0
            } else {
                1
            }
        } else {
            if priority::is_ge(ordering) {
                0
            } else {
                1
            }
        };
    }
    if ascending {
        if priority::is_lt(compare(item, &sequence[0])) {
            return 0;
        } else if priority::is_gt(compare(item, &sequence[length-1])) {
            return length;
        }
    } else {
        if priority::is_gt(compare(item, &sequence[0])) {
            return 0;
        } else if priority::is_lt(compare(item, &sequence[length-1])) {
            return length;
        }
    }
    let mut left = 1;
    let mut right = length - 1;
    // Put the this order check outside the while loop so that it runs
    // slightly faster.
    if ascending {
        while left <= right {
            let middle = left + (right-left)/2;
            let ordering = compare(item, &sequence[middle]);
            if priority::is_eq(ordering) {
                return left;
            } else if priority::is_lt(ordering) {
                right = middle-1;
            } else {
                left = middle+1;
            }
        }
    } else {
        while left <= right {
            let middle = left + (right-left)/2;
            let ordering = compare(item, &sequence[middle]);
            if priority::is_eq(ordering) {
                return left;
            } else if priority::is_gt(ordering) {
                right = middle-1;
            } else {
                left = middle+1;
            }
        }
    }
    left
}

/// Find where an `item` should be in an ordered `sequence`. This function
/// checks to see if the sequence has been ordered properly or not, If the
/// sequence is unsorted, `Err` is returned.
/// 
/// If the slice is correctly sorted, 2 possibilities may arise:
/// 1. An element with the same priority as `item` is found,
/// 2. No matching element is found in the slice but the hypothetical
///    location for `item` if it were to be in the slice is found.
/// 
/// If scenario 1 happens `Ok(Ok(location))` is returned, but
/// if scenario 2 happens `Ok(Err(location))` is returned.
/// If `item` is greater than the last element in the `sequence`,
/// `Ok(Err(sequence.as_ref().len()))` is returned.
/// 
/// # Examples
/// 
/// This is the result for a correctly ordered array.
/// 
/// ```
///     use algocol::binarysearch::binarysearch;
///     let array = [0, 2, 4, 6, 8];
///     let location = binarysearch(&array[..], &5, true);
///     assert_eq!(location, Ok(Err(3))); // If 5 were to be inserted into
///                             // `array` while making sure that the array
///                             // remains sorted, 5 should be placed at index
///                             // 3.
/// ```
/// 
/// However, this is what happens if the array is not sorted,
/// 
/// ```
///     use algocol::{binarysearch::binarysearch};
///     let array = [0, 8, 2, 6, 4];
///     let location = binarysearch(&array[..], &5, true);
///     assert!(matches!(location, Err(_)))
/// ```
/// 
/// `None` is returned as the array is not sorted and the function doesn't
/// know that where 5 should be placed.
pub fn binarysearch<S, T>(
    sequence: &S,
    item: &T,
    ascending: bool
) -> AgcResult<Result<usize, usize>>
where
    S: AsRef<[T]> + ?Sized,
    T: Ord
{
    let sequence = sequence.as_ref();
    if !is_sorted(sequence, ascending) {
        return Err(
            AgcError::new(AgcErrorKind::Unordered, "sequence is not sorted.")
        );
    }
    let location = binarysearch_unchecked(sequence, item, ascending);
    if priority::eq(item, &sequence[location]) {
        Ok(Ok(location))
    } else {
        Ok(Err(location))
    }
}

/// Find where an `item` should be in an ordered `sequence`. This function
/// checks to see if the sequence has been ordered properly or not, If the
/// sequence is unsorted, `Err` is returned. This function requires a
/// function to compare two elements together, the function should be passed
/// as the argument for the parameter called `compare`.
/// 
/// If the slice is correctly sorted, 2 possibilities may arise:
/// 1. An element with the same priority as `item` is found,
/// 2. No matching element is found in the slice but the hypothetical
///    location for `item` if it were to be in the slice is found.
/// 
/// If scenario 1 happens `Ok(Ok(location))` is returned, but
/// if scenario 2 happens `Ok(Err(location))` is returned.
/// If `item` is greater than the last element in the `sequence`,
/// `Ok(Err(sequence.as_ref().len()))` is returned.
/// 
/// # Examples
/// 
/// This is the result for a correctly ordered array.
/// 
/// ```
///     use algocol::binarysearch::binarysearch_by;
///     let array = [0, 2, 4, 6, 8];
///     let location = binarysearch_by(&array[..], &5, true, |a, b| a.cmp(b));
///     assert_eq!(location, Ok(Err(3))); // If 5 were to be inserted into
///                             // `array` while making sure that the array
///                             // remains sorted, 5 should be placed at index
///                             // 3.
/// ```
/// 
/// However, this is what happens if the array is not sorted,
/// 
/// ```
///     use algocol::binarysearch::binarysearch_by;
///     let array = [0, 8, 2, 6, 4];
///     let location = binarysearch_by(&array[..], &5, true, |a, b| a.cmp(b));
///     assert!(matches!(location, Err(_)))
/// ```
/// 
/// `None` is returned as the array is not sorted and the function doesn't
/// know that where 5 should be placed.
pub fn binarysearch_by<F, S, T>(
    sequence: &S,
    item: &T,
    ascending: bool,
    compare: F
) -> AgcResult<Result<usize, usize>>
where
    S: AsRef<[T]> + ?Sized,
    F: Fn(&T, &T) -> Ordering + Copy
{
    let sequence = sequence.as_ref();
    if !is_sorted_by(sequence, ascending, compare) {
        return Err(
            AgcError::new(AgcErrorKind::Unordered, "sequence is not sorted.")
        );
    }
    let location = binarysearch_unchecked_by(
        sequence,
        item,
        ascending, 
        compare
    );
    if priority::is_eq(compare(item, &sequence[location])) {
        Ok(Ok(location))
    } else {
        Ok(Err(location))
    }
}