extern crate num_traits;

use std::u32;
use std::env;
use std::fmt::Display;
use std::ops::Add;
use num_traits::bounds::Bounded;

fn overflow<T>(i: T)
    where T: Bounded + Add + Copy + Display, <T as Add>::Output: Display
{
    println!("{} + {} = {}", T::max_value(), i, T::max_value() + i);
}

fn main() {
    let i: u32 = env::args().nth(1).unwrap_or(String::from("1")).parse().unwrap_or(1);
    overflow(i);
    let i: i32 = i as i32;
    overflow(i);
    let i: u64 = i as u64;
    overflow(i);
    let i: i64 = i as i64;
    overflow(i);
}
