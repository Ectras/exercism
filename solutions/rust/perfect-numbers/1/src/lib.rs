use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factor_sum(num: u64) -> u64 {
    if num == 1 {
        return 0;
    }

    let mut sum = 1;
    for x in 2..=num / 2 {
        if num % x == 0 {
            sum += x;
        }
    }
    sum
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    match num.cmp(&factor_sum(num)) {
        Ordering::Less => Some(Classification::Abundant),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Deficient),
    }
}
