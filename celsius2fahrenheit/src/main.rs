use std::io;
use std::io::Write;

fn main() {
    let mut celsius = String::new();

    print!("Celsius: ");
    io::stdout().flush()
        .expect("Failed to flush");

    io::stdin().read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Bad value: {}", err);
            return;
        }
    };

    let fahrenheit = celsius * 9./5. + 32.;

    println!("Fahrenheit: {:.2}°F", fahrenheit);
}
