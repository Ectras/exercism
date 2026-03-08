use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RULES: Regex = Regex::new(
        r"(?x)^
    (?:
        # Rule 1
        () (?:[aeiuo] | xr | yt)

        # Rule 4
    |   ([^aeiou]+) y

        # Rule 3
    |   ([^aeiou]* qu)

        # Rule 2
    |   ([^aeiou]+)
    )
    .*  # rest of the word",
    )
    .unwrap();
}

fn translate_word(word: &str) -> String {
    let caps = RULES.captures(word).unwrap();
    let consonants = (1..=4)
        .map(|i| caps.get(i))
        .filter_map(|x| x)
        .next()
        .unwrap();
    String::from(&word[consonants.end()..]) + consonants.as_str() + "ay"
}

pub fn translate(input: &str) -> String {
    let words = input
        .split_ascii_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>();
    words.join(" ")
}
