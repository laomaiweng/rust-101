use std::i32;

fn main() {
    let x = {
        let x = 3;
        x*2
    };
    let y = if x > 5 {
        five()
    } else {
        plus_one(five())
    };
    //let xx: i32 = (1 << 31) - 1;  // fails to compile due to overflow
    let xx: i32 = i32::MAX-3;

    let a = ["foo", "bar", "baz"];
    for s in a.iter().rev() {
        println!("{}", s);
    }

    let mut cd: i32 = y;
    while cd >= 0 {
        println!("{}", cd);
        cd = minus_one(cd);
    }
    println!("IGNITION");

    for i in 1..4 {
        println!("{}", i);
    }
    println!("LIFTOFF");

    another_function(plus_one(x), y, xx);
}

fn another_function(x: i32, y: i32, z: i32) {
    let mut z = z;
    loop {
        println!("The value of x is: {}", x);
        if y == 5 {
            println!("The value of y is: {}", y);
        }
        z = plus_one(z);
        println!("The value of z is: {}", z);   // triggers overflow, works just as well with u32 instead of i32
        println!("LOOP!");
    }
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn minus_one(x: i32) -> i32 {
    x - 1
}
