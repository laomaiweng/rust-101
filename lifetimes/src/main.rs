use std::fmt;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> fmt::Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.part)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, string2);
    println!("The longest string is: {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next()
        .expect("Could not find a '.'");
    let excerpt = ImportantExcerpt{ part: first_sentence };
    println!("{}", excerpt);
}
