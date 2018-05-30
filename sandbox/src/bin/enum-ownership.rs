enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    if std::env::args().count() != 2 {
        println!("Usage: enum-ownership <coin>");
        return;
    }
    let coin = match std::env::args().nth(1) {
        Some(c) => match c.to_lowercase().as_ref() {
                "penny" => Coin::Penny,
                "nickel" => Coin::Nickel,
                "dime" => Coin::Dime,
                "quarter" => Coin::Quarter,
                _ => {
                    println!("unknown coin");
                    return;
                }
            },
        _ => Coin::Penny
    };
    let coin = match std::env::args().nth(1) {
        Some(c) => match c.to_lowercase().as_ref() {
                "penny" => Coin::Penny,
                "nickel" => Coin::Nickel,
                "dime" => Coin::Dime,
                "quarter" => Coin::Quarter,
                _ => {
                    println!("unknown coin value");
                    return;
                }
            },
        _ => Coin::Penny
    };

    println!("value in cents: {}", value_in_cents(coin));
    //println!("value in cents: {}", value_in_cents(coin));
}

fn value_in_cents(coin: Coin) -> i64 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
