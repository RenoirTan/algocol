//! Contains definition of traits that `algocol` uses.
//! 
//! Algocol uses some traits to constrain certain algorithms to types which
//! implement them. This module defines traits which `algocol` functions
//! use.
//! 
//! Just like the definitions in `algocol::error`, the traits here are
//! prepended with `Agc`.

use num_traits::{NumOps, NumAssignOps};
use std::hash::Hash;

/// `AgcNumberLike` is a trait that is automatically implemented on types which
/// have implemented number traits such as `std::ops::Add` or
/// `std::ops::SubAssign`. Such number-like types must also be comparable
/// with each other. Hence, they must also implement traits like `Ord` or `Eq`.
/// To allow for easy movement of data, `Copy` and `Clone` must also be
/// implemented.
/// 
/// Here is the full list of traits that must be implemented for your type
/// in order to implement `AgcNumberLike`:
/// 
/// 1. `Add<Output=Self>`
/// 2. `Sub<Output=Self>`
/// 3. `Mul<Output=Self>`
/// 4. `Div<Output=Self>`
/// 5. `Rem<Output=Self>`
/// 6. `AddAssign`
/// 7. `SubAssign`
/// 8. `MulAssign`
/// 9. `DivAssign`
/// 10. `RemAssign`
/// 11. `Ord`
/// 12. `PartialOrd`
/// 13. `Eq`
/// 14. `PartialEq`
/// 15. `Copy`
/// 16. `Clone`
/// 17. `Sized` (so no slices or trait objects)
pub trait AgcNumberLike:
      NumOps
    + NumAssignOps
    + Ord
    + PartialOrd
    + Eq
    + PartialEq
    + Copy
    + Clone
    + Sized
{}

impl<T> AgcNumberLike for T
where
    T: NumOps
    + NumAssignOps
    + Ord
    + PartialOrd
    + Eq
    + PartialEq
    + Copy
    + Clone
    + Sized
{}

/// `AgcHash` is implemented on types which implement `std::hash::Hash`,
/// `PartialEq` and `Eq`. This implies that the type can be used in hash
/// functions or hash collections such as `std::collections::HashMap`.
pub trait AgcHashable: Hash + PartialEq + Eq {}

impl<T: Hash + PartialEq + Eq> AgcHashable for T {}