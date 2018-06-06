#[macro_use] extern crate lazy_static;
extern crate regex;

pub mod noregex;
// TODO: mod tests;

enum WordType {
    VowelStarting,
    ConsonantStarting,
    Other
}

pub fn pig_latin(text: &str) -> String {
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
