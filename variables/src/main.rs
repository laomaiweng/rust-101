fn main() {
    let x: u128 = 0x1337u128;
    let b = b'\x20';
    let s = "😻";

    println!("{}{} {}", b, s, s.len());

    let mut t: (u128, &str, bool, f32) = (x, "foo", true, 3.14);
    let t0 = t.0;
    let t1 = t.1;
    let t2 = t.2;
    t.2 = false;
    let t3 = t.3;

    println!("{} {} {} {}", t0, t1, t2, t3);

    let a = [1, 2, 3];

    println!("{}", a[5]);
}
