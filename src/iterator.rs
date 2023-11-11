pub fn iterator_main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for v in v1_iter {
        println!("Got: {}", v);
    }
}

#[test]
fn iterator_test() {
    let v = vec![1, 2, 3];
    let mut it = v.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), None);
}
