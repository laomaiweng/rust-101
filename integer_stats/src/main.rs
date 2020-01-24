use std::collections::HashMap;
use std::env;

fn main() {
    // gather arguments as integers
    let ints: Vec<i128> = env::args().skip(1).map(|s| s.parse().expect(&format!("not an int: {}", s))).collect();
    assert!(! ints.is_empty(), "no integers");

    println!("mean: {}", mean(&ints));
    println!("median: {}", median(&ints));
    println!("mode: {:?}", mode(&ints));
}

fn mean(ints: &Vec<i128>) -> f64 {
    let sum: i128 = ints.iter().sum();
    (sum as f64) / (ints.len() as f64)
}

fn median(ints: &Vec<i128>) -> i128 {
    let mut ints = ints.clone();
    ints.sort();

    let imed = if ints.len() % 2 == 0 {
        ints.len() / 2 - 1
    } else {
        ints.len() / 2
    };

    ints[imed]
}

fn mode(ints: &Vec<i128>) -> Vec<i128> {
    let mut ints = ints.clone();
    ints.sort();

    let mut count: HashMap<i128, usize> = HashMap::new();
    for i in &ints {
        *count.entry(*i).or_insert(0) += 1;
    }

    let mut rcount: HashMap<usize, Vec<i128>> = HashMap::new();
    let mut max: usize = 0;
    for (i, c) in &count {
        rcount.entry(*c).or_insert(Vec::new()).push(*i);
        if *c > max {
            max = *c;
        }
    }

    rcount.get(&max).unwrap().clone()
}
