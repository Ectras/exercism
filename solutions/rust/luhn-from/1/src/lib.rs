pub struct Luhn(String);

impl Luhn {
    fn double(x: u32) -> u32 {
        let mut x = 2 * x;
        if x > 9 {
            x -= 9;
        }
        x
    }

    pub fn is_valid(&self) -> bool {
        let total: Option<Vec<u32>> = self
            .0
            .as_bytes()
            .iter()
            .filter(|&&c| c != b' ')
            .rev()
            .map(|&c| char::to_digit(c as char, 10))
            .enumerate()
            .map(|(i, c)| if i % 2 == 1 { c.map(Self::double) } else { c })
            .collect();
        total.map_or(false, |x| x.len() > 1 && x.iter().sum::<u32>() % 10 == 0)
    }
}

impl<T> From<T> for Luhn where T: ToString {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
