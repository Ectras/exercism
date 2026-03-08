pub fn encode(source: &str) -> String {
    // Collect the runs
    let mut runs = Vec::new();
    for &c in source.as_bytes() {
        if let Some((ch, count)) = runs.last_mut() {
            if c == *ch {
                *count += 1;
            } else {
                runs.push((c, 1));
            }
        } else {
            runs.push((c, 1));
        }
    }

    // Build the string
    let mut out = String::new();
    for (ch, count) in runs {
        if count > 1 {
            out.push_str(&count.to_string());
        }
        out.push(char::from_u32(ch.into()).unwrap());
    }
    out
}

pub fn decode(source: &str) -> String {
    let mut out = String::new();
    let mut current_factor = None;
    for c in source.chars() {
        match c {
            '0'..='9' => {
                // Got a digit
                let d = char::to_digit(c, 10).unwrap();
                if let Some(ref mut fac) = current_factor {
                    // We are in the process of reading a number -> add digit at the end
                    *fac = 10 * (*fac) + d;
                } else {
                    // We are not reading a number -> start reading a number with this digit
                    current_factor = Some(d)
                }
            }
            _ => {
                if let Some(fac) = current_factor.take() {
                    // We have read some number previously, so append this char multiple times
                    for _ in 0..fac {
                        out.push(c);
                    }
                } else {
                    // No number read previously, just append the char once
                    out.push(c);
                }
            }
        }
    }
    out
}
