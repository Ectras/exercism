use std::fmt::{Display, Formatter, Result, Write};

pub struct Roman {
    digits: Vec<u8>,
}

impl Roman {
    fn print_digit(&self, f: &mut Formatter<'_>, mut digit: u8, chars: &[char]) -> Result {
        if digit == 9 {
            f.write_char(chars[0])?;
            f.write_char(chars[2])?;
        } else if digit == 4 {
            f.write_char(chars[0])?;
            f.write_char(chars[1])?;      
        }
        else {
            if digit >= 5 {
                f.write_char(chars[1])?;
                digit -= 5;
            }
            for _ in 0..digit {
                f.write_char(chars[0])?;
            }
        }
        Ok(())
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const SYMBOLS: [char; 9] = ['I', 'V', 'X', 'L', 'C', 'D', 'M', '_', '_'];
        let mut lower = 2 * (self.digits.len() - 1);
        for &digit in &self.digits {
            self.print_digit(f, digit, &SYMBOLS[lower..=lower+2])?;
            lower = lower.saturating_sub(2);
        }
        Ok(())
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mut digits = Vec::new();
        while num > 0 {
            let d = (num % 10) as u8;
            digits.push(d);
            num /= 10;
        }
        digits.reverse();
        Self { digits }
    }
}