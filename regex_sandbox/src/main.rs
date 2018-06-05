extern crate regex;

use std::io;
use std::io::Write;
use regex::Regex;

fn main() {
    loop {
        print!("RE: ");
        io::stdout().flush()
            .expect("failed to flush");
        let mut re = String::new();
        io::stdin().read_line(&mut re)
            .expect("failed to read line");

        let re = Regex::new(&re).expect("bad regex");

        println!("Text (single line):");
        let mut text = String::new();
        io::stdin().read_line(&mut text)
            .expect("failed to read line");

        println!("{:?}", re.captures(&text));
        println!("");
    }
}
