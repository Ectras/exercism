pub fn reverse(input: &str) -> String {
    let mut s = String::from(input);
    unsafe {
        let vec = s.as_mut_vec();
        vec.reverse();
    }
    s
}
