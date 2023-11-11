use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, A>
where
    T: Fn(A) -> A,
    A: Eq + Hash + Copy,
{
    calculation: T,
    values: HashMap<A, A>,
}

impl<T, A> Cacher<T, A>
where
    T: Fn(A) -> A,
    A: Eq + Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, A> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> A {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_closure.value(intensity));
        println!("Next, do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure.value(intensity)
            );
        }
    }
}

pub fn closure_main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod cacher_tests {
    use super::*;
    #[test]
    fn test_cacher1() {
        let mut cacher = Cacher::new(|v| v);
        let v1 = cacher.value(1);
        assert_eq!(v1, 1);
    }

    #[test]
    fn test_cacher2() {
        let mut cacher = Cacher::new(|v| v);
        cacher.value(1);
        let v2 = cacher.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn test_cacher_with_usize() {
        let arg: usize = 1;
        let mut cacher = Cacher::new(|v| v);
        let v = cacher.value(arg);
        assert_eq!(arg, v);
    }
}

#[cfg(test)]
mod closure_test {
    #[test]
    fn test1() {
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));
    }
}
