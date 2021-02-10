//! Utility functions on slices.

use crate::error::{AgcError, AgcResult, AgcErrorKind};

pub use transfer_element as sl_move;

/// Move an element in a slice to another part of the slice.
/// This is done by shifting the elements before or after the slice (depending
/// on where the element came `from` to the left or right, making space for
/// the target element to move `to`.
/// This function returns an error if `from` or `to` are out of bounds.
///
///  # Example
/// ```
///     use algocol::utils::slice::transfer_element;
///     let mut array: [i32; 5] = [0, 1, 2, 3, 4];
///     transfer_element(&mut array[..], 4, 1).unwrap();
///     assert_eq!(array, [0, 4, 1, 2, 3]);
/// ```
pub fn transfer_element<T>(
    slice: &mut [T],
    from: usize,
    to: usize
) -> AgcResult<()> {
    let length = slice.len();
    if from >= length || to >= length {
        return Err(
            AgcError::new(
                AgcErrorKind::OutOfBounds,
                "from and to must be smaller than the length of the slice."
            )
        );
    }
    if from == to {
        return Ok(());
    } else if from < to {
        slice[from..=to].rotate_left(1);
    } else {
        slice[to..=from].rotate_right(1);
    }
    Ok(())
}