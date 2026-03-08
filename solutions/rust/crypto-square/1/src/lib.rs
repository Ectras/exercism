use std::iter::once;

fn rect(len: usize) -> (usize, usize) {
    let sq = (len as f32).sqrt();
    let rows = sq.floor() as usize;
    let cols = sq.ceil() as usize;
    (rows, cols)
}

pub fn encrypt(input: &str) -> String {
    // Normalize the string
    let normalized = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>();

    // Find the rectangle
    let (rows, cols) = rect(normalized.len());

    // Write the groups
    let mut out = String::with_capacity(rows * cols);
    for col in 0..cols {
        if col > 0 {
            out.push(' ');
        }
        let group = normalized
            .chars()
            .skip(col)
            .step_by(cols)
            .chain(once(' '))
            .take(rows);
        out.extend(group);
    }
    out
}
