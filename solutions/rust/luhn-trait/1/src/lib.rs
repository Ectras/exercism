pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

fn double(x: u32) -> u32 {
    let mut x = 2 * x;
    if x > 9 {
        x -= 9;
    }
    x
}

impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let total: Option<Vec<u32>> = self
            .to_string()
            .as_bytes()
            .iter()
            .filter(|&&c| c != b' ')
            .rev()
            .map(|&c| char::to_digit(c as char, 10))
            .enumerate()
            .map(|(i, c)| if i % 2 == 1 { c.map(double) } else { c })
            .collect();
        total.map_or(false, |x| x.len() > 1 && x.iter().sum::<u32>() % 10 == 0)
    }
}
