#[derive(Debug)]
enum List<'a, T: 'a> {
    Cons(T, &'a List<'a, T>),
    Nil,
}

impl<'a, T: 'a> List<'a, T> {
    fn insert(&mut self, e: T, i: usize) {
        match self {
            Cons(_, ref mut s) => {
                if i > 0 {
                    s.insert(e, i-1);   // don't have a mutable reference to the nested Cons
                } else {
                    *s = &Cons(e, *s);  // can't make this live long enough
                }
            },
            Nil => panic!("Trying to insert on Nil!"),
        }
    }
}

use List::{Cons, Nil};

fn main() {
    let nil = &Nil;
    let d = Cons(10, &nil);
    let a = Cons(5, &d);
    let e = Cons(4, &a);
    let b = Cons(3, &e);
    let c = Cons(2, &a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}
