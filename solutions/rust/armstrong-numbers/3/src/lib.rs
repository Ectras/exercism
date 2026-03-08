pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len() as u32;
    let mut sum = 0u32;
    for d in num_str.chars().map(|c| c.to_digit(10).unwrap())
    {
        let Some(dp) = d.checked_pow(num_len) else { return false };
        let Some(new_sum) = sum.checked_add(dp) else { return false };
        sum = new_sum;
    }
    sum
}
