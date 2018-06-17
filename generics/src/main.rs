#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> T {
        self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<i32> {
    fn distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct ConfusedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> ConfusedPoint<T, U> {
    fn mixup<V, W>(self, other: ConfusedPoint<V, W>) -> ConfusedPoint<T, W> {
        ConfusedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T>(list: &[T]) -> T {
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

    let ipt_list = vec![Point{ x: 5, y: 10 }, Point{ x:-3, y: 30 }, Point{ x:9, y:8 }];
    let result = largest(&ipt_list);
    println!("The largest integer point is {:?}", result);

    let fpt_list = vec![Point{ x: 5., y: 10. }, Point{ x:-3., y: 30. }, Point{ x:9., y:8. }];
    let result = largest(&fpt_list);
    println!("The largest floating point point is {:?}", result);

    let p1 = ConfusedPoint{ x: 5, y: 10.4 };
    let p2 = ConfusedPoint{ x: "Hello", y: 'c' };
    let p3 = p2.mixup(p1);
    println!("mixup({:?}, {:?}) = {:?}", p1, p2, p3);
}
