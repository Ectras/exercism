pub fn plural_suffix(n: u32) -> &'static str {
    match n {
        1 => "",
        _ => "s",
    }
}

pub fn number_word(n: u32) -> String {
    match n {
        0 => "no more".to_owned(),
        _ => n.to_string()
    }
}

// https://stackoverflow.com/a/61285863
fn str_cap(s: &str) -> String {
  format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

pub fn verse(n: u32) -> String {
    let (current, action, remaining) = match n {
        0 => (0, "Go to the store and buy some more", 99),
        1 => (1, "Take it down and pass it around", 0),
        _ => (n, "Take one down and pass it around", n-1),
    };

    let current_bottles = format!("{} bottle{} of beer", number_word(current), plural_suffix(current));
    let remaining_bottles = format!("{} bottle{} of beer", number_word(remaining), plural_suffix(remaining));
    let current_bottles_upper = str_cap(current_bottles.as_str());
    format!("{current_bottles_upper} on the wall, {current_bottles}.\n{action}, {remaining_bottles} on the wall.\n")
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
