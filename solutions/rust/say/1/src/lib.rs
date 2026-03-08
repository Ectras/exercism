fn rev_split_digits(mut n: u64) -> Vec<u16> {
    let mut out = Vec::new();
    while n > 0 {
        out.push((n % 1000) as u16);
        n /= 1000;
    }
    out
}

fn get_single(n: u16) -> String {
    const WORDS: [&str; 19] = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelv",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    WORDS[(n - 1) as usize].into()
}

fn get_tens(n: u16) -> String {
    const WORDS: [&str; 8] = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    WORDS[(n - 2) as usize].into()
}

fn encode_double(n: u16) -> String {
    match n {
        1..=19 => get_single(n),
        20..=99 => {
            let mut out = get_tens(n / 10);
            if n % 10 > 0 {
                out += "-";
                out += &get_single(n % 10);
            }
            out
        }
        _ => unreachable!(),
    }
}

fn encode_sub_thousand(n: u16) -> String {
    match n {
        1..=99 => encode_double(n),
        100..=999 => {
            let mut out = get_single(n / 100) + " hundred";
            let d2 = n % 100;
            if d2 > 0 {
                out.push(' ');
                out.push_str(&encode_double(d2));
            }
            out
        }
        _ => unreachable!(),
    }
}

pub fn encode(n: u64) -> String {
    // Special case zero
    if n == 0 {
        return "zero".into();
    }

    const SCALE_WORDS: [&str; 6] = [
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    let mut out = String::new();
    for (i, chunk) in rev_split_digits(n)
        .iter()
        .enumerate()
        .filter(|(_, &ch)| ch != 0)
        .rev()
    {
        if !out.is_empty() {
            out.push(' ');
        }
        out += &encode_sub_thousand(*chunk as u16);
        if i > 0 {
            out.push(' ');
            out.push_str(SCALE_WORDS[i - 1]);
        }
    }
    out
}
