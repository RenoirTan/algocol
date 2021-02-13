use algocol::alreadysorted;

#[test]
fn test_alreadysorted() {
    use algocol::alreadysorted;
    let sequence = [0; 1];
    let sorted = alreadysorted!(bool sequence.len());
    println!("alreadysorted result: {}", sorted);
}