//! Quicksort functions
//! 
//! **Currently not working**

use std::{
    cmp::{Ord, Ordering},
    convert::AsMut
};
use crate::{
    alreadysorted,
    error::{AgcResult, AgcError, AgcErrorKind},
    utils::priority
};

/// The partition function used in quicksort. It takes a pivot element in the
/// `sequence` and moves the elements smaller than the pivot to the front of
/// the sequence and the elements larger than the pivot to the back of the
/// sequence assuming that you are sorting in ascending order. `left` is the
/// index of the first element in the slice and `right` is the length of the
/// slice of the `sequence` you want to partition.
/// `compare` is the function used to check whether 2 elements are smaller,
/// equal to or greater than each other.
/// 
/// # Example
/// ```
///     use algocol::sort::quicksort::partition;
///     let mut sequence = [10, 80, 30, 90, 40, 50, 70];
///     partition(&mut sequence, 0, 7, true, |a, b| a.cmp(b)).unwrap();
///     assert_eq!(sequence, [10, 30, 40, 50, 70, 90, 80]);
/// ```
pub fn partition<F, S, T>(
    sequence: &mut S,
    left: usize,
    right: usize,
    ascending: bool,
    compare: F
) -> AgcResult<usize>
where
    S: AsMut<[T]> + ?Sized,
    F: Fn(&T, &T) -> Ordering + Copy
{
    let sequence = sequence.as_mut();
    let length = sequence.len();
    alreadysorted!(result length, return 0);
    if left > right {
        return Err(AgcError::new(
            AgcErrorKind::WrongOrder,
            format!(
                "Left ({}) must be less than or equal to right ({})",
                left,
                right
            )
        ));
    } else if left >= length {
        return Err(AgcError::new(
            AgcErrorKind::OutOfBounds,
            format!("Left ({}) must be less than length ({})", left, length)
        ));
    } else if right > length {
        return Err(AgcError::new(
            AgcErrorKind::OutOfBounds,
            format!(
                "Right ({}) must be less than or equal to length ({})",
                right,
                length
            )
        ));
    }
    // -1 because `right` is the index after the last element in the slice
    let pivot = right - 1;
    // println!();
    // println!("New partition called! Pivot: {}", pivot);
    // Where I got the idea to name these 2 variables as such:
    // https://www.youtube.com/watch?v=pKO9UjSeLew
    //
    // `tortoise` and `hare` correspond to `i` and `j` in GeeksforGeeks
    // quicksort article (https://www.geeksforgeeks.org/quick-sort/)
    //
    // `tortoise` is the location of the last element whose priority is less
    // than the pivot (if ascending) and
    // `hare` is the current index being checked
    // As `hare` is guaranteed to always increase faster than `tortoise`,
    // it means that `hare` is always after (or equal to) `tortoise`.
    // Hence, if there are any descrepancies between their priorities,
    // like if hare has a lower priority than the pivot, then `tortoise` and
    // `hare` can be swapped. Since the lower priority element has moved to
    // `tortoise`, it means that `tortoise` will still be pointing at an
    // element that is smaller than the pivot. `tortoise` is incremented to
    // make space for more swaps without causing the last smaller element to
    // get swapped to the position where `hare` is pointing at.
    let mut tortoise = left;
    for hare in left..pivot {
        // println!("tortoise: {}", tortoise);
        // println!("hare: {}", hare);
        let ordering = compare(&sequence[hare], &sequence[pivot]);
        if (priority::is_le(ordering) && ascending)
        || (priority::is_ge(ordering) && !ascending) {
            sequence.swap(tortoise, hare);
            tortoise += 1;
        }
    }
    // Put the pivot element after the last element smaller than pivot.
    sequence.swap(tortoise, pivot);
    Ok(tortoise)
}

/// Sort a slice using the quicksort algorithm. The algorithm picks a pivot in
/// the slice and puts the items smaller than it to the left of it and those
/// larger than it to the right of it. The slice then gets split in 2, the
/// former is before the pivot while the second resides after the pivot. Each
/// subslice then gets partitioned into smaller and smaller slices until the
/// original slice is sorted.
/// 
/// # Example
/// ```
///    use algocol::sort::quicksort::quicksort;
///    let mut sequence = (0..100).collect::<Vec<i32>>();
///    sequence.reverse();
///    quicksort(
///        &mut sequence[..], true
///    ).unwrap();
///    assert_eq!(sequence, (0..100).collect::<Vec<i32>>());
/// ```
pub fn quicksort<S, T>(
    sequence: &mut S,
    ascending: bool
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    T: Ord
{
    quicksort_by(sequence, ascending, |a, b| a.cmp(b))
}

/// Sort a slice using the quicksort algorithm. The algorithm picks a pivot in
/// the slice and puts the items smaller than it to the left of it and those
/// larger than it to the right of it. The slice then gets split in 2, the
/// former is before the pivot while the second resides after the pivot. Each
/// subslice then gets partitioned into smaller and smaller slices until the
/// original slice is sorted.
/// 
/// This function requires a `compare` function to work.
/// 
/// # Example
/// ```
///    use algocol::sort::quicksort::quicksort_by;
///    let mut sequence = (0..100).collect::<Vec<i32>>();
///    sequence.reverse();
///    quicksort_by(
///        &mut sequence[..], true, |a, b| a.cmp(b)
///    ).unwrap();
///    assert_eq!(sequence, (0..100).collect::<Vec<i32>>());
/// ```
pub fn quicksort_by<F, S, T>(
    sequence: &mut S,
    ascending: bool,
    compare: F
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    F: Fn(&T, &T) -> Ordering + Copy
{
    
    struct SegmentPair {
        pub start: usize,
        pub end: usize
    };

    let sequence = sequence.as_mut();
    let length = sequence.len();
    alreadysorted!(result length, return sequence);

    // `stack` stores the segments of the sequences yet to be partitioned
    let mut stack: Vec<SegmentPair> = Vec::new();
    stack.push(SegmentPair {start: 0, end: length-1});
    // If there are still segments to be partitioned
    while let Some(segment) = stack.pop() {
        let pivot = partition(
            sequence,
            segment.start,
            segment.end+1,
            ascending,
            compare
        )?;
        // If the pivot is in the middle of the segment, then push the 2
        // subsegments
        if pivot > segment.start + 1 {
            stack.push(SegmentPair {start: segment.start, end: pivot-1});
        }
        if pivot + 1 < segment.end {
            stack.push(SegmentPair {start: pivot + 1, end: segment.end});
        }
    }
    Ok(sequence)
}

/// Sort a slice using the quicksort algorithm. The algorithm picks a pivot in
/// the slice and puts the items smaller than it to the left of it and those
/// larger than it to the right of it. The slice then gets split in 2, the
/// former is before the pivot while the second resides after the pivot. Each
/// subslice then gets partitioned into smaller and smaller slices until the
/// original slice is sorted. This function is recursive.
/// 
/// # Example
/// ```
///    use algocol::sort::quicksort::quicksort_recursively;
///    let mut sequence = (0..100).collect::<Vec<i32>>();
///    sequence.reverse();
///    quicksort_recursively(
///        &mut sequence[..], true
///    ).unwrap();
///    assert_eq!(sequence, (0..100).collect::<Vec<i32>>());
/// ```
pub fn quicksort_recursively<S, T>(
    sequence: &mut S,
    ascending: bool
) -> AgcResult<&mut [T]>
where
    S: AsMut<[T]> + ?Sized,
    T: Ord
{
    quicksort_recursively_by(sequence, ascending, |a, b| a.cmp(b))
}

/// Sort a slice using the quicksort algorithm. The algorithm picks a pivot in
/// the slice and puts the items smaller than it to the left of it and those
/// larger than it to the right of it. The slice then gets split in 2, the
/// former is before the pivot while the second resides after the pivot. Each
/// subslice then gets partitioned into smaller and smaller slices until the
/// original slice is sorted. This function is recursive.
/// 
/// This function requires a `compare` function to work.
/// 
/// # Example
/// ```
///    use algocol::sort::quicksort::quicksort_recursively_by;
///    let mut sequence = (0..100).collect::<Vec<i32>>();
///    sequence.reverse();
///    quicksort_recursively_by(
///        &mut sequence[..], true, |a, b| a.cmp(b)
///    ).unwrap();
///    assert_eq!(sequence, (0..100).collect::<Vec<i32>>());
/// ```
pub fn quicksort_recursively_by<F, S, T>(
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
    let pivot = partition(sequence, 0, length, ascending, compare)?;
    quicksort_recursively_by(&mut sequence[..pivot], ascending, compare)?;
    quicksort_recursively_by(&mut sequence[pivot+1..], ascending, compare)?;
    Ok(sequence)
}