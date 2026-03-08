pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut include_next = true;
    let mut maybe_include_next = false;
    for ch in phrase.chars() {
        if maybe_include_next {
            include_next = char::is_ascii_uppercase(&ch);
            maybe_include_next = false;
        }

        if include_next {
            if char::is_ascii_alphabetic(&ch) {
                acronym.push(char::to_ascii_uppercase(&ch));
                include_next = false;
            }
        } else {
            match ch {
                '-' | ' ' => include_next = true,
                'a'..='z' => maybe_include_next = true,
                _ => (),
            };
        }
    }
    acronym
}
