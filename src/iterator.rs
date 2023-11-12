pub fn iterator_main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for v in v1_iter {
        println!("Got: {}", v);
    }
}

#[cfg(test)]
mod iterator_test {
    #[test]
    fn iterator_test() {
        let v = vec![1, 2, 3];
        let mut it = v.iter();
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
    }

    fn sum_test() {
        let v = vec![1, 2, 3];
        let it = v.iter();
        let s: i32 = it.sum();
        assert_eq!(s, 6);
    }

    fn map() {
        let v = vec![1, 2, 3];
        let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
        assert_eq!(v2, [2, 3, 4]);
    }

    fn filter() {
        let v = vec![10, 2, 10];
        let v2: Vec<_> = v.iter().filter(|&&x| x == 10).map(|&x| x).collect();
        assert_eq!(v2, [10, 10]);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn counter_test() {
    let c = Counter::new();
    let v: Vec<_> = c.collect();
    assert_eq!(v, [1, 2, 3, 4, 5]);
}
