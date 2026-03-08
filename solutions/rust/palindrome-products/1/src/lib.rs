/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if Self::is_palindrome(Self::to_digits(value)) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }

    fn to_digits(number: u64) -> Vec<u8> {
        if number == 0 {
            return vec![0];
        }

        let mut out = Vec::new();
        let mut n = number;
        while n > 0 {
            out.push((n % 10) as u8);
            n /= 10;
        }
        out.reverse();
        out
    }

    fn is_palindrome(digits: Vec<u8>) -> bool {
        for i in 0..digits.len() / 2 {
            if digits[i] != digits[digits.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut extremas = None;
    for a in min..max {
        for b in a..=max {
            if let Some(pal) = Palindrome::new(a * b) {
                let (min, max) = extremas.get_or_insert((pal, pal));
                if pal.0 < min.0 {
                    *min = pal;
                }

                if pal.0 > max.0 {
                    *max = pal;
                }
            }
        }
    }

    extremas
}
