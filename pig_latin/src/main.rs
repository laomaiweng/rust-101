#[macro_use] extern crate log;
extern crate env_logger;
extern crate pig_latin;

use std::io;
use std::collections::HashMap;

type Engine = fn(&str) -> String;

fn main() {
    env_logger::init();

    // consume stdin
    let mut isascii = true;
    let mut lines: Vec<String> = vec![];
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("failed to read line");
        if line.is_empty() {
            break;
        }
        isascii &= line.is_ascii();
        lines.push(line);
    }

    // run with all engines
    let mut engines: HashMap<&str, Engine> = HashMap::new();
    engines.insert("regex", pig_latin::pig_latin);
    engines.insert("noregex", pig_latin::noregex::pig_latin);
    for (label, func) in &engines {
        println!("=== Engine: {} ===", label);
        for line in &lines {
            print!("{}", func(&line));
        }
        println!("");
    }

    if !isascii {
        warn!("Non-ascii characters found in text! Translation may not be accurate.");
    }
}
