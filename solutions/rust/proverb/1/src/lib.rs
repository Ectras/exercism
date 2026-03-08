pub fn build_proverb(list: &[&str]) -> String {
    let mut iter = list.iter();
    if let Some(first) = iter.next() {
        let mut out = String::new();
        let mut current = first;
        for next in iter {
            out.push_str(format!("For want of a {} the {} was lost.\n", current, next).as_str());
            current = next;
        }
        out.push_str(format!("And all for the want of a {}.", first).as_str());
        out
    } else {
        String::new()
    }
}
