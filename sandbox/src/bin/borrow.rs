fn main() {
    let mut s = "foo".to_string();
    let rs = &s;

    println!("{}", s);
    println!("{}", rs);

    //s.push_str("bar");
    //s = String::from("baz");

    {
        let s = "foobar".to_string();
        println!("{}", s);
        println!("{}", rs);
    }
}
