pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let digits = s.len() as u32;
    num == s.chars().map(|c| c.to_digit(10).unwrap().pow(digits)).sum::<u32>()
}
