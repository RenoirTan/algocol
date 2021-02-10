#[test]
fn test_binarysearch_unchecked() {
    use algocol::binarysearch::binarysearch_unchecked;
    let array = [0, 2, 4, 6, 8];
    let mut location: usize = 0;
    if location == 0 {}
    location = binarysearch_unchecked(&array[..], &5, true);
    println!("Where 5 should be: {}", location);
    location = binarysearch_unchecked(&array[..], &-1, true);
    println!("Where -1 should be: {}", location);
    location = binarysearch_unchecked(&array[..], &9, true);
    println!("Where 9 should be: {}", location);
    location = binarysearch_unchecked(&array[..], &0, true);
    println!("Where 0 should be: {}", location);
    location = binarysearch_unchecked(&array[..], &8, true);
    println!("Where 8 should be: {}", location);
    location = binarysearch_unchecked(&array[..], &1, true);
    println!("Where 1 should be: {}", location);
    location = binarysearch_unchecked(&array[..], &7, true);
    println!("Where 7 should be: {}", location);
}