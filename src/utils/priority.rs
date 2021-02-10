//! This module defines functions which determine the precedence or priority of
//! 2 or more items of the same type.

#![allow(dead_code)]

use std::{
    cmp::{Ord, Ordering}
};

/// Checks to see if `a` is bigger than `b`.
#[inline]
pub (crate) fn gt<T: Ord>(a: &T, b: &T) -> bool {
    matches!(a.cmp(b), Ordering::Greater)
}

/// Checks to see if `a` is greater than or equal to `b`.
#[inline]
pub (crate) fn ge<T: Ord>(a: &T, b: &T) -> bool {
    matches!(a.cmp(b), Ordering::Greater | Ordering::Equal)
}

/// Checks to see if `a` has takes the same precedence as `b`.
#[inline]
pub (crate) fn eq<T: Ord>(a: &T, b: &T) -> bool {
    matches!(a.cmp(b), Ordering::Equal)
}

/// Checks to see if `a` is less than `b`.
#[inline]
pub (crate) fn lt<T: Ord>(a: &T, b: &T) -> bool {
    matches!(a.cmp(b), Ordering::Less)
}

/// Checks to see if `a` is less than or takes the same precedence as `b`.
#[inline]
pub (crate) fn le<T: Ord>(a: &T, b: &T) -> bool {
    matches!(a.cmp(b), Ordering::Less | Ordering::Equal)
}

/// `true` if order is `Ordering::Less`.
pub fn is_lt(order: Ordering) -> bool {
    matches!(order, Ordering::Less)
}

/// `true` if order is `Ordering::Less` or `Ordering::Equal`.
pub fn is_le(order: Ordering) -> bool {
    matches!(order, Ordering::Less | Ordering::Equal)
}

/// `true` if order is `Ordering::Equal`.
pub fn is_eq(order: Ordering) -> bool {
    matches!(order, Ordering::Equal)
}

/// `true` if order is `Ordering::Greater` or `Ordering::Equal`.
pub fn is_ge(order: Ordering) -> bool {
    matches!(order, Ordering::Greater | Ordering::Equal)
}

/// `true` if order is `Ordering::Greater`.
pub fn is_gt(order: Ordering) -> bool {
    matches!(order, Ordering::Greater)
}