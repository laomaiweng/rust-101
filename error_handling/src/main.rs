use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::process;

fn main() {
    if env::args().len() != 2 {
        println!("usage: error_handling <file>");
        process::exit(1);
    }

    let fname = env::args().nth(1).unwrap();
    println!("{}", read_first_line(&fname).unwrap());
}

fn read_first_line(filename: &str) -> Result<String, io::Error> {
    let mut f = match fs::File::open(filename) {
        Ok(file) => file,
        Err(ref error) if error.kind() == io::ErrorKind::NotFound => {
            fs::OpenOptions::new().create(true).write(true).read(true).open(filename)?
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        },
    };
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(String::from(s.split('\n').next().unwrap_or("")))
}
