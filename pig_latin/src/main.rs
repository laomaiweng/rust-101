#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use std::io;

fn main() {
    pretty_env_logger::init();

    //let pig_latin = pig_latin_cbc;
    let pig_latin = pig_latin_re;

    let mut isascii = true;

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("failed to read line");
        if line.is_empty() {
            break;
        }

        print!("{}", pig_latin(&line));
        isascii &= line.is_ascii();
    }

    if !isascii {
        warn!("Non-ascii characters found in text! Translation may not be accurate.");
    }
}

fn pig_latin_cbc(text: &str) -> String {
    // output
    let mut output = String::new();

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

    output
}

enum WordType {
    VowelStarting,
    ConsonantStarting,
    Other
}

fn pig_latin_re(text: &str) -> String {
    lazy_static! {
        static ref RE_WORD: regex::Regex = regex::Regex::new(r"\b\w{2,}\b(?:-\w+\b)*").unwrap();    // capture words, not counting a single dash as a word separator
    }

    let text = RE_WORD.replace_all(&text, |caps: &regex::Captures| {
        let mut chars = caps[0].chars();
        let first_letter = chars.next().unwrap();   // guaranteed by regex
        let word_type = match first_letter.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => WordType::VowelStarting,
            'a'...'z' => WordType::ConsonantStarting,
            _ => WordType::Other,
        };
        let second_letter = chars.next().unwrap();  // guaranteed by regex
        let second_letter = match word_type {
            WordType::ConsonantStarting if first_letter.is_uppercase() => second_letter.to_uppercase().collect(),
            _ => second_letter.to_string(),
        };
        let other_letters: String = chars.collect();
        match word_type {
            WordType::VowelStarting => format!("{}{}{}-hay", first_letter, second_letter, other_letters),
            WordType::ConsonantStarting => format!("{}{}-{}ay", second_letter, other_letters, first_letter.to_lowercase()),
            _ => format!("{}{}{}", first_letter, second_letter, other_letters),
        }
    });

    String::from(text)
}
