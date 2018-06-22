struct Counter {
    count: u32
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

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    // .into_iter() to return owned values
    // .iter_mut() to return mutable references

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    for val in v1_iter {    // takes ownership of v1_iter and mutates it behind the scene, no need for mut
        println!("Got: {}", val);
    }

    let mut v1_iter = v1.iter();
    while let Some(v) = v1_iter.next() {    // modifies v1_iter but does not take ownership, needs mut
        println!("Got: {}", v);
    }

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));

    let v1_iter = v1.iter();
    assert_eq!(6, v1_iter.sum());   // takes ownership of v1_iter

    let v1_iter = v1.iter();
    let v2: Vec<_>  = v1_iter.map(|x| x+1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    let shoes = vec![
        Shoe { size:10, style: String::from("sneaker") },
        Shoe { size:13, style: String::from("sandal") },
        Shoe { size:10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("in my size: {:?}", in_my_size);

    let mut counter = Counter::new();
    while let Some(v) = counter.next() {
        println!("Counter: {}", v);
    }

    let res: u32 = Counter::new().skip(1).zip(Counter::new()).map(|(a,b)| a*b).filter(|x| x % 3 == 0).sum();
    println!("convoluted: {}", res);
}
