use std::env;
use std::process;

fn main() {
    if env::args().len() != 2 && env::args().len() != 3 {
        println!("usage: fibonacci <n> [implem]");
        println!("");
        println!("implementations: naive tail iter calc");
        process::exit(1);
    }

    let n: u128 = match env::args().nth(1).unwrap().trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Bad value: {}", err);
            return;
        }
    };

    // see https://stackoverflow.com/questions/48034119/rust-matching-a-optionstring
    let implem = match env::args().nth(2).as_ref().map(|s| s.as_str()) {
        Some("naive") => fibonacci_naive,
        Some("tail") => fibonacci_tail,
        Some("iter") => fibonacci_iter,
        Some("calc") => fibonacci_calc,
        Some(i) => {
            println!("Bad implementation: {}", i);
            process::exit(1);
        },
        None => fibonacci_iter,
    };

    println!("{}", implem(n));
}

fn fibonacci_naive(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_naive(n-2) + fibonacci_naive(n-1),
    }
}



fn fibonacci_tail(n: u128) -> u128 {
    fn fib(n: u128, a: u128, b: u128) -> u128 {
        match n {
            0 => a,
            _ => fib(n-1, b, a + b),
        }
    }
    fib(n, 0, 1)
}

fn fibonacci_iter(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut fib = (0, 1);
    for _ in 1..n {
        fib = (fib.1, fib.0 + fib.1);
    }
    fib.1
}

fn fibonacci_calc(n: u128) -> u128 {
    let phi = (1. + (5. as f64).sqrt()) / 2.;
    ((phi.powi(n as i32) - phi.recip().powi(n as i32)) / (5. as f64).sqrt()).round() as u128
}
