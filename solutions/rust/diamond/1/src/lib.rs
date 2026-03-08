use std::iter::{once, repeat};

fn number(c: char) -> u8 {
    c as u8 - b'A' + 1
}

fn padding(count: u8) -> impl Iterator<Item = char> {
    repeat(' ').take(count.into())
}

pub fn get_diamond(c: char) -> Vec<String> {
    let c_num = number(c);
    let mut lines = Vec::with_capacity((2 * c_num - 1).into());
    for d in ('A'..=c).chain(('A'..c).rev()) {
        let d_num = number(d);
        let outer_padding = c_num - d_num;
        let inner_padding = (2 * (d_num - 1)).saturating_sub(1);

        let line = padding(outer_padding)
            .chain(once(d))
            .chain(
                padding(inner_padding)
                    .chain(once(d))
                    .take_while(|_| inner_padding > 0),
            )
            .chain(padding(outer_padding))
            .collect();
        lines.push(line);
    }
    lines
}
