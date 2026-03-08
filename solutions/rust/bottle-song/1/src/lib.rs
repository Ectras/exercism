pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn get_word(count: usize) -> (&'static str, &'static str) {
        const COUNTS: [&str; 11] = ["No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
        let plural = if count != 1 { "s" } else { "" };
        (COUNTS[count], plural)
    }
    let mut out = String::new();
    for count in (0..=start_bottles as _).rev().take(take_down as _) {
        if !out.is_empty() {
            out += "\n\n";
        }
        let (current, cs) = get_word(count);
        let (next, ns) = get_word(count - 1);
        let next = next.to_lowercase();
        out += &format!("\
{current} green bottle{cs} hanging on the wall,
{current} green bottle{cs} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {next} green bottle{ns} hanging on the wall.\
        ");
    }
    out
}


