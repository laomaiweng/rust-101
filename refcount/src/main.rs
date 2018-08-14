use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("-- count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::new(Cons(4, Rc::clone(&a))));
    println!("-- count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(2, Rc::clone(&a));
        println!("-- count after creating c = {}", Rc::strong_count(&a));
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    }
    println!("-- count after dropping c = {}", Rc::strong_count(&a));
}
