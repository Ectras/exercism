pub fn find<R, T>(array: R, key: T) -> Option<usize>
where
    R: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = left + (right - 1 - left) / 2;
        if array[mid] < key {
            // We need to go to the right
            left = mid + 1;
        } else if array[mid] > key {
            // We need to go to the left
            right = mid.saturating_sub(1);
        } else {
            // We found it
            return Some(mid);
        }
    }
    None
}
