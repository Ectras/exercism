use std::{
    borrow::Cow,
    ops::{Add, Mul, Sub},
};

use num_bigint::{BigInt, Sign};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal {
    // implement your type here
    mantissa: BigInt,
    /// This is the negative exponent to base 10. Meaning the mantissa must be
    /// multiplied with `10^{-exponent}`.
    exponent: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut expects_sign = true;
        let mut expects_dot = true;
        let mut reads_significant = false;

        let mut sign = Sign::Plus;
        let mut integer_digit_count = 0;
        let mut digits = Vec::new();

        for c in input.chars() {
            match c {
                '+' | '-' if expects_sign => {
                    if c == '-' {
                        sign = Sign::Minus
                    };
                    expects_sign = false;
                }
                '.' if expects_dot => {
                    reads_significant = true;
                    integer_digit_count = digits.len();
                    expects_dot = false;
                }
                '0' if reads_significant => {
                    digits.push(c.to_digit(10).unwrap() as u8);
                }
                '0' => continue,
                '1'..='9' => {
                    digits.push(c.to_digit(10).unwrap() as u8);
                    reads_significant = true;
                }
                _ => return None,
            };
        }

        let exponent = if expects_dot {
            0
        } else {
            digits.len() - integer_digit_count
        };

        let mantissa = BigInt::from_radix_be(sign, &digits, 10).unwrap();
        let mut number = Decimal { mantissa, exponent };
        number.normalize();

        Some(number)
    }

    fn normalize(&mut self) {
        if self.mantissa == BigInt::ZERO {
            self.exponent = 0;
            return;
        }

        while self.exponent > 0 && &self.mantissa % 10 == BigInt::ZERO {
            self.mantissa /= 10;
            self.exponent -= 1;
        }
    }

    fn denormalized(&self, new_exponent: usize) -> Cow<BigInt> {
        assert!(new_exponent >= self.exponent);
        if self.exponent == new_exponent {
            Cow::Borrowed(&self.mantissa)
        } else {
            let base: BigInt = 10.into();
            let exponent = new_exponent - self.exponent;
            let shift = base.pow(exponent.try_into().unwrap());
            let new = &self.mantissa * shift;
            Cow::Owned(new)
        }
    }

    fn perform_binop(&self, rhs: &Decimal, op: fn(&BigInt, &BigInt) -> BigInt) -> Decimal {
        let max_exp = self.exponent.max(rhs.exponent);
        let a = self.denormalized(max_exp);
        let b = rhs.denormalized(max_exp);
        let c = op(&a, &b);
        let mut out = Decimal {
            mantissa: c,
            exponent: max_exp,
        };
        out.normalize();
        out
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let max_exp = self.exponent.max(other.exponent);
        let a = self.denormalized(max_exp);
        let b = other.denormalized(max_exp);
        a.cmp(&b)
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Self::Output {
        self.perform_binop(&rhs, |a, b| a + b)
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        self.perform_binop(&rhs, |a, b| a - b)
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        let mantissa = self.mantissa * rhs.mantissa;
        let exponent = self.exponent + rhs.exponent;
        let mut out = Decimal { mantissa, exponent };
        out.normalize();
        out
    }
}
