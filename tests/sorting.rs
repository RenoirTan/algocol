extern crate algocol;

#[test]
fn test_bubblesort() {
    use algocol::sort::bubblesort;
    let mut sequence: [i32; 3] = [3,2,1];
    let result = bubblesort::bubblesort(&mut sequence[..], true);
    println!("bubblesort result: {:?}", result);
    println!("bubblesort: {:?}", sequence);
    assert_eq!(sequence, [1,2,3]);
}

#[test]
fn test_selectionsort() {
    use algocol::sort::selectionsort;
    let mut sequence: [i32; 5] = [5,4,3,2,1];
    let result = selectionsort::selectionsort(&mut sequence[..], true);
    println!("selectionsort result: {:?}", result);
    println!("selectionsort: {:?}", sequence);
    assert_eq!(sequence, [1,2,3,4,5]);
}

#[test]
fn test_insertionsort() {
    use algocol::sort::insertionsort;
    let mut sequence: [i32; 5] = [1,2,3,4,5];
    let result = insertionsort::insertionsort(&mut sequence[..], false);
    println!("insertionsort result: {:?}", result);
    println!("insertionsort: {:?}", sequence);
    assert_eq!(sequence, [5,4,3,2,1]);
}

#[test]
fn test_mergesort() {
    use algocol::sort::mergesort;
    let mut sequence: [i32; 5] = [5,4,3,2,1];
    let result = mergesort::mergesort(&mut sequence[..], true);
    println!("mergesort result: {:?}", result);
    println!("mergesort: {:?}", sequence);
    assert_eq!(sequence, [1,2,3,4,5]);
}

#[test]
fn test_merge() {
    use algocol::sort::mergesort::merge;
    let mut array = [7, 6, 1, 3, 6, 2, 4, 5, 8, 20];
    let result = merge(&mut array[..], 2, 4, 8, true, |a, b| a.cmp(b));
    println!("merge result: {:?}", result);
    assert_eq!(array, [7, 6, 1, 2, 3, 4, 5, 6, 8, 20]);
}

#[test]
fn test_mergesort_recursive() {
    use algocol::sort::mergesort;
    let mut sequence: [i32; 5] = [5,4,3,2,1];
    let result = mergesort::mergesort_recursively(&mut sequence[..], true);
    println!("mergesort_recursive result: {:?}", result);
    println!("mergesort_recursive: {:?}", sequence);
    assert_eq!(sequence, [1,2,3,4,5]);
}

#[test]
fn test_timsort() {
    use algocol::sort::timsort::{timsort_by, DEFAULT_RUN};
    let mut sequence = (0..100).collect::<Vec<i32>>();
    sequence.reverse();
    let result = timsort_by(
        &mut sequence[..], true, DEFAULT_RUN, |a, b| a.cmp(b)
    );
    println!("timsort result: {:?}", result);
    println!("timsort: {:?}", sequence);
    assert_eq!(sequence, (0..100).collect::<Vec<i32>>());
}

#[test]
fn test_quicksort() {
    use algocol::sort::quicksort::quicksort_by;
    let mut sequence = (0..100).collect::<Vec<i32>>();
    sequence.reverse();
    let result = quicksort_by(
        &mut sequence[..], true, |a, b| a.cmp(b)
    );
    println!("quicksort result: {:?}", result);
    println!("quicksort: {:?}", sequence);
    assert_eq!(sequence, (0..100).collect::<Vec<i32>>());
}

#[test]
fn test_quicksort_recursive() {
    use algocol::sort::quicksort::quicksort_recursively;
    let mut sequence = (0..100).collect::<Vec<i32>>();
    sequence.reverse();
    let result = quicksort_recursively(
        &mut sequence[..], true
    );
    println!("quicksort_recursive result: {:?}", result);
    println!("quicksort_recursive: {:?}", sequence);
    assert_eq!(sequence, (0..100).collect::<Vec<i32>>());
}

#[test]
fn test_partition() {
    use algocol::sort::quicksort::partition;
    let mut sequence = [10, 80, 30, 90, 40, 50, 70];
    let result = partition(&mut sequence, 0, 7, true, |a, b| a.cmp(b));
    println!("quicksort result: {:?}", result);
    println!("quicksort: {:?}", sequence);
    assert_eq!(sequence, [10, 30, 40, 50, 70, 90, 80]);
    assert!(matches!(result, Ok(4)));
}