use std::thread;
use std::collections::HashMap;
use std::time::Duration;
use std::hash::Hash;

struct Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Eq + Hash + Copy,
          V: Copy,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Eq + Hash + Copy,
          V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        let Cacher { ref calculation, ref mut values } = self;
        *values.entry(arg).or_insert_with(|| (calculation)(arg))
        // must use unpacking since can't borrow self through self.calculation while borrowed mutably through self.values
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    //println!("x was: {:?}", x);   // can't use x any more
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    assert_eq!(v1, 1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}
