//! Stores definitions of macros used by this crate.

/// Check if a slice is already sorted by checking if the length of the slice
/// less than or equal to 1. If so, it runs an expression.
/// 
/// This macro can be used in constant functions.
/// 
/// # Example
/// 
/// ```
///     use algocol::alreadysorted;
///     pub const fn is_sorted<T>(slice: &[T]) -> bool {
///         already_sorted!(slice.len() boolean)
///     }
/// ```
#[macro_export]
macro_rules! alreadysorted {
    () => {
        return Ok(());
    };
    (result $length: expr) => {
        if $length <= 1 {
            return Ok(());
        }
    };
    (result $length: expr, return $value: expr) => {
        if $length <= 1 {
            return Ok($value);
        }
    };
    (bool $length: expr) => {
        $length <= 1
    };
    ($length: expr, $then: expr) => {
        if $length <= 1 {$then}
    }
}

/// Convert `Option<T>` into `T` if `Some(T)` or return
/// `Err(AgcError)` if `None`.
#[macro_export]
macro_rules! agctryoption {
    ($option: expr) => {
        match option {
            Some(thing) => thing,
            None => return Err($crate::AgcError::new(
                $crate::AgcErrorKind::NotFound,
                "Could not be found."
            ))
        }
    };
    ($option: expr, $desc: expr) => {
        match option {
            Some(thing) => thing,
            None => return Err($crate::AgcError::new(
                $crate::AgcErrorKind::NotFound,
                $desc
            ))
        }
    }
}