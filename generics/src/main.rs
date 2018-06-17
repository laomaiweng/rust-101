#![feature(specialization)]

extern crate num_traits;

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
{
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

trait Norm<T> {
    fn norm(&self) -> T;
}

impl<T> Norm<T> for Point<T>
    where T: num_traits::Num + Copy
{
    default fn norm(&self) -> T {
        self.x * self.y
    }
}

//TODO: breaks, why?
//impl Norm<u32> for Point<u32> {
//    fn norm(&self) -> u32 {
//        self.x * self.y
//    }
//}

impl<T> Norm<T> for Point<T>
    where T: num_traits::Signed + Copy
{
    default fn norm(&self) -> T {
        self.x.abs() + self.y.abs()
    }
}

impl Norm<f32> for Point<f32> {
    fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> PartialOrd for Point<T>
    where T: PartialOrd + num_traits::Num + Copy
{
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        self.norm().partial_cmp(&other.norm())
    }
}

#[derive(Debug, Copy, Clone)]
struct ConfusedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> ConfusedPoint<T, U>
    where T: Copy
{
    fn mixup<V, W>(&self, other: &ConfusedPoint<V, W>) -> ConfusedPoint<T, W> 
        where W: Copy
    {
        ConfusedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T>(list: &[T]) -> T 
    where T: PartialOrd + Copy
{
    let mut result = list[0];
    for &item in list.iter() {
        if item > result {
            result = item;
        }
    }

    result
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let ipt_list = vec![Point{ x: 5, y: 10 }, Point{ x:-3, y: 20 }, Point{ x:9, y:8 }];
    let result = largest(&ipt_list);
    println!("The largest integer point is {:?}", result);

    let upt_list = vec![Point{ x: 5u32, y: 10u32 }, Point{ x:3u32, y: 20u32 }, Point{ x:9u32, y:8u32 }];
    let result = largest(&upt_list);
    println!("The largest unsigned integer point is {:?}", result);

    let fpt_list = vec![Point{ x: 5., y: 10. }, Point{ x:-3., y: 20. }, Point{ x:9., y:8. }];
    let result = largest(&fpt_list);
    println!("The largest floating point point is {:?}", result);

    let p1 = ConfusedPoint{ x: 5, y: 10.4 };
    let p2 = ConfusedPoint{ x: "Hello", y: 'c' };
    let p3 = p2.mixup(&p1);
    println!("mixup({:?}, {:?}) = {:?}", p1, p2, p3);
}
