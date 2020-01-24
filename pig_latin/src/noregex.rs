pub fn pig_latin(text: &str) -> String {
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
        // TODO: this doesn't handle multiple dashes the same way the regex implementation does
        if c.is_none() || c.unwrap().is_whitespace() || (c.unwrap().is_ascii_punctuation() && c.unwrap() != '-') {
            // if word too short, don't transform it, and restore its first character
            let elision_len = match elision {
                Some(_) => 1,
                None => 0,
            };
            if word.len() + elision_len < 2 {   // use word.len() only gives us the character count if input is ASCII
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
                'a'..='z' => {
                    if suffix.is_empty() {
                        // setup consonant suffix (char moved => must update case)
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
