#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut row_0 = vec![0; (max_weight + 1).try_into().unwrap()];
    let mut row_1 = vec![0; row_0.len()];
    for i in 0..items.len() {
        for w in 1..=max_weight {
            row_1[w as usize] = if items[i].weight > w {
                row_0[w as usize]
            } else {
                u32::max(
                    row_0[w as usize],
                    row_0[(w - items[i].weight) as usize] + items[i].value,
                )
            };
        }
        std::mem::swap(&mut row_0, &mut row_1);
    }
    *row_0.last().unwrap()
}
