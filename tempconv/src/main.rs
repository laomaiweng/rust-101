use std::io;
use std::io::Write;

const F2C_FACTOR: f64 = 5./9.;
const F2C_OFFSET: f64 = -32.;
const K2C_OFFSET: f64 = -273.15;

fn main() {
    println!("===== Temperature converter =====");
    println!("=                               =");
    println!("= Choose you conversion:        =");
    println!("= 1) °F -> °C                   =");
    println!("= 2) °C -> °F                   =");
    println!("= 3) °K -> °C                   =");
    println!("= 4) °C -> °K                   =");
    println!("= 5) °K -> °F                   =");
    println!("= 6) °F -> °K                   =");
    println!("=================================");

    let choice = loop {
        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse::<usize>() {
            Ok(num) => {
                if 0 < num && num < 7 {
                    break num - 1
                } else {
                    continue;
                }
            },
            Err(_) => continue,
        };
    };

    let converters = [fahrenheit2celsius, celsius2fahrenheit, kelvin2celsius, celsius2kelvin, kelvin2fahrenheit, fahrenheit2kelvin];
    let input = ["Fahrenheit", "Celsius", "Kelvin", "Celsius", "Kelvin", "Fahrenheit"];
    let output = ["Celsius", "Fahrenheit", "Celsius", "Kelvin", "Fahrenheit", "Kelvin"];

    print!("{}: ", input[choice]);
    io::stdout().flush()
        .expect("Failed to flush");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Bad value: {}", err);
            return;
        }
    };

    println!("{}: {:.2}", output[choice], converters[choice](input));
}

fn fahrenheit2celsius(t: f64) -> f64 {
    (t + F2C_OFFSET) * F2C_FACTOR
}

fn celsius2fahrenheit(t: f64) -> f64 {
    t / F2C_FACTOR - F2C_OFFSET
}

fn kelvin2celsius(t: f64) -> f64 {
    t + K2C_OFFSET
}

fn celsius2kelvin(t: f64) -> f64 {
    t - K2C_OFFSET
}

fn kelvin2fahrenheit(t: f64) -> f64 {
    celsius2fahrenheit(kelvin2celsius(t))
}

fn fahrenheit2kelvin(t: f64) -> f64 {
    celsius2kelvin(fahrenheit2celsius(t))
}
