extern crate algocol;

fn generate_unsorted(from: i128, to: i128, ascending: bool) -> Vec<i128> {
    let mut vector = (if from <= to {from..to} else {to..from})
        .collect::<Vec<i128>>();
    if !ascending {
        vector.reverse();
    }
    vector
}

fn default_unsorted() -> Vec<i128> {
    generate_unsorted(0, 100, false)
}

#[test]
fn experiment_partition() {
    use algocol::sort::quicksort::partition;
    let mut array = default_unsorted();
    let length = array.len();
    let result = partition(&mut array[..], 0, length, true, |a, b| a.cmp(b));
    println!("(1) partition result: {:?}", result);
    println!("(1) partition array: {:?}", array);
}