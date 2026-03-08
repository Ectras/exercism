use std::cmp::Ordering;

pub fn find<R, T>(array: R, key: T) -> Option<usize>
where
    R: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = (left + right) / 2;
        match array[mid].cmp(&key) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => right = mid,
        }
    }
    None
}
