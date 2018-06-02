fn main() {
    main_ints();
    main_tuple();
    main_tuplestruct();
    main_struct();
}

fn main_ints() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_ints(width1, height1)
    );
}

fn area_ints(width: u32, height: u32) -> u64 {
    (width as u64) * (height as u64)
}

fn main_tuple() {
    let rect1 = (30, 50);

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
}

fn area_tuple(r: (u32, u32)) -> u64 {
    (r.0 as u64) * (r.1 as u64)
}

#[derive(Debug)]
struct RectangleTS(u32, u32);

fn main_tuplestruct() {
    let rect1 = RectangleTS(30, 50);

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuplestruct(&rect1)
    );
}

fn area_tuplestruct(r: &RectangleTS) -> u64 {
    (r.0 as u64) * (r.1 as u64)
}

#[derive(Debug)]
struct RectangleS {
    width: u32,
    height: u32
}

impl RectangleS {
    fn area(&self) -> u64 {
        (self.width as u64) * (self.height as u64)
    }

    fn is_bigger(&self, other: &RectangleS) -> bool {
        self.area() > other.area()
    }

    fn can_hold(&self, other: &RectangleS) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl RectangleS {
    fn square(size: u32) -> RectangleS {
        RectangleS { width: size, height: size }
    }
}

fn main_struct() {
    let rect1 = RectangleS { width: 30, height: 60 };
    let rect2 = RectangleS { width: 10, height: 40 };
    let rect3 = RectangleS { width: 70, height: 45 };
    let rect4 = RectangleS { width: 70, height: 20 };
    let square = RectangleS::square(40);

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Is rect1 bigger than rect2? {}", rect1.is_bigger(&rect2));
    println!("Is rect1 bigger than rect3? {}", rect1.is_bigger(&rect3));
    println!("Is rect1 bigger than rect4? {}", rect1.is_bigger(&rect4));
    println!("Is rect1 bigger than square? {}", rect1.is_bigger(&square));

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}

fn area_struct(r: &RectangleS) -> u64 {
    (r.width as u64) * (r.height as u64)
}
