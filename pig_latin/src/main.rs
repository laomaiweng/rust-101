#[macro_use] extern crate log;
extern crate pretty_env_logger;

use std::io;

fn main() {
    pretty_env_logger::init();

    let mut nonascii: Vec<char> = vec![];

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("failed to read line");
        if line.is_empty() {
            break;
        }

        let (output, line_nonascii) = pig_latin(&line);
        print!("{}", &output);
        nonascii.extend(line_nonascii);
    }

    if !nonascii.is_empty() {
        warn!("Non-ascii characters in text: {}", nonascii.iter().collect::<String>());
    }
}

fn pig_latin(text: &str) -> (String, Vec<char>) {
    // output
    let mut output = String::new();
    let mut nonascii: Vec<char> = vec![];

    // word state
    let mut elision: Option<char> = None;
    let mut word = String::new();
    let mut suffix = String::new();
    let mut next_char_to_upper = false;

    // loop over all chars
    // don't use `for c in chars` since we want the ending None to be handled in the loop
    let mut chars = text.chars();
    loop {
        // get the next char
        let c = chars.next();

        // log non-ascii chars
        if let Some(ch) = c {
            if !ch.is_ascii() {
                nonascii.push(ch);
            }
        }

        // word boundaries: EOF/whitespace/punctuation (except dash)
        if c.is_none() || c.unwrap().is_whitespace() || (c.unwrap().is_ascii_punctuation() && c.unwrap() != '-') {
            // if word too short, don't transform it, and restore its first character
            if word.len() < 2 {
                if !suffix.is_empty() {
                    suffix.clear();
                    if let Some(ch) = elision {
                        word = format!("{}{}", ch, word);
                    }
                }
            }
            // append word, its suffix, the current character
            let ch = match c {
                Some(c) => c.to_string(),
                None => String::new()
            };
            output += &format!("{}{}{}", word, suffix, ch);
            // reset word state
            word.clear();
            suffix.clear();
            elision = None;
            // continue or EOF
            if c.is_none() {
                break;
            } else {
                continue;
            }
        } else {
            // we're not None
            let c = c.unwrap();
            // uppercase char if needed for transformation
            let ch: String = if next_char_to_upper {
                next_char_to_upper = false;
                c.to_uppercase().collect()
            } else {
                c.to_string()
            };
            // handle character according to vowel > consonant > everything else
            match c.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' | 'y' => {
                    if suffix.is_empty() {
                        // setup vowel suffix (no char moved => no case to change)
                        suffix = String::from("-hay");
                    }
                    word.push_str(&ch);
                },
                'a'...'z' => {
                    if suffix.is_empty() {
                        // setup consonant suffix (char moved => case to update)
                        suffix = format!("-{}ay", c.to_lowercase());
                        elision = Some(c);
                        next_char_to_upper = c.is_uppercase();
                    } else {
                        word.push_str(&ch);
                    }
                },
                _ => {
                    word.push_str(&ch);
                },
            }
        }
    }

    (output, nonascii)
}
