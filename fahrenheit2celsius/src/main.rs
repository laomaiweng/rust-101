use std::io;
use std::io::Write;

fn main() {
    let mut fahrenheit = String::new();

    print!("Fahrenheit: ");
    io::stdout().flush()
        .expect("Failed to flush");

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Bad value: {}", err);
            return;
        }
    };

    let celsius = (fahrenheit - 32.) * 5./9.;

    println!("Celsius: {:.2}°C", celsius);
}
