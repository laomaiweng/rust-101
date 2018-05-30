use std::fmt;

struct User {
    username: String,
    email: String,
    signin_count: u64,
    active: bool
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let active = match self.active {
            true => "active",
            false => "inactive"
        };
        write!(f, "{} <{}> ({})", self.username, self.email, active)
    }
}

fn main() {
    let email = String::from("bar");
    let u1 = User {
        username: "foo".to_string(),
        email,
        signin_count: 0,
        active: false
    };
    let mut u2 = User {
        username: String::from("baz"),
        email: String::from("foobar"),
        ..u1
    };

    println!("{}", u1);
    u2.active = true;
    println!("{}", u2);
}
