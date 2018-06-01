use std::env;
use std::process;

fn main() {
    if env::args().len() != 2 {
        println!("usage: fibonacci <n>");
        process::exit(1);
    }

    let n: u128 = match env::args().last().unwrap().trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Bad value: {}", err);
            return;
        }
    };

    println!("{}", fibonacci(n));
}

fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-2) + fibonacci(n-1),
    }
}
