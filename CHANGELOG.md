# algocol v0.2.3

## *By Renoir Tan*

--------------------------------------------------------------------------------

## Version 0.1.0

First version of `algocol`. Available in this version:

1. Bubble Sort
2. Selection Sort
3. Custom Error and Result types
4. Utils for operating on slices and types which can be `Ord`ered.

## Version 0.1.1

Add reexports for `algocol`.

## Version 0.2.0

Added

1. Insertion Sort
2. Merge Sort
3. Binary Search
4. Functions to resolve `Ordering` into booleans depending on whether they are
less than, less than or equal to, equal to, greater than or equal to, or
greater than.

## Version 0.2.1

Add re-exports for algorithms with explanations on the affixes used by the
new re-exported names.

Some functions in this crate have been re-exported with shortened names
with useful classification affixes in the format `<prefix>_<name>_<suffixes>`.
For example, `algocol::sort::mergesort_recursively_by` is re-exported as
`s_merge_rf`. The `s` prefix means that this function sorts a sequence
such as a slice. `merge` is the algorithm that the function uses to sort
the slice. `rf` is actually a compound suffix. `r` means that the function
is recursive and `f` means that a function must be provided as an
argument for a task used by our mergesort function, in this case, the
input function is called `compare` and returns the `std::cmp::Ordering`
between 2 elements.

Below are the prefixes currently used by this crate as of version **0.2.1**:

1. `s`: For functions which sort slices
2. `sc`: Functions which search for an element in a sequence
3. `sl`: Utility functions on slices

The following suffix parts are used in this crate as of version **0.2.1**:

1. `i`: This function is iterative (as opposed to recursive)
2. `r`: This function is recursive
3. `f`: This function requires an auxiliary function

## Version 0.2.2

1. Added `timsort`.
2. Fixed `mergesort_recursive` and `mergesort_recursively_by`.

## Version 0.2.3

1. Added `quicksort` and its associated `partition` function.
2. Modified `algocol::alreadysorted` to return an `Ok($value)` if the
   sequence is already sorted using this syntax:

   ```rust
   alreadysorted!(result length, return something);
   ```

   The `result` at the start tells the macro that if the length of the slice
   is 1 or less, return `Ok(something)`.
3. Use bitshift left instead of multiply to marginally improve performance.
