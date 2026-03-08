use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    roman: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.roman)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let romap = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut num = num;
        let mut roman = String::new();
        for (k, v) in &romap {
            println!("{}", k);
            while num >= *k {
                println!("{} {}", num, *k);
                roman += v;
                num -= *k;
            }
        }

        Self { roman }
    }
}