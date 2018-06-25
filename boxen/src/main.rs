use std::ops::{Deref, DerefMut};

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!")
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_mut(name: &mut String) {
    name.push_str("Mut");
    hello(name);
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("list: {:?}", list);

    let x = 5;
    let y = &x;
    let z = 5;
    println!("x = {}", x);
    println!("*y = {}", *y);
    println!("y = {} :(", y);
    println!("z = {}", z);
    println!("x == *y: {}", x == *y);
    println!("&x == y: {}", &x == y);
    println!("&x == &z: {} :(", &x == &z);
    println!("&x as *const i32 == &z as *const i32: {}", &x as *const i32 == &z as *const i32);
    let zz = Box::new(z);
    println!("x == *Box(z): {}", x == *zz);
    //println!("&x == Box(z): {}", &x == zz);

    {
        let zzz = MyBox::new(*zz);
        drop(zzz);
        let zzz = MyBox::new(z);
        println!("x == *MyBox(z): {}", x == *zzz);
    }

    let h = MyBox::new(String::from("Deref"));
    hello(&h);

    let mut h = MyBox::new(String::from("Deref"));
    hello_mut(&mut h);
}
