use std::{thread, time};

struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name.to_lowercase())
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn panic() {
        panic!("Failed panicking test!");
    }

    #[test]
    #[should_panic]
    fn successful_panic() {
        panic!("Successful panicking test!");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was: {}", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    #[ignore]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn sleepy_head() {
        let three_secs = time::Duration::from_millis(3000);
        let now = time::Instant::now();

        thread::sleep(three_secs);

        assert!(now.elapsed() >= three_secs);
    }
}
