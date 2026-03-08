use std::iter::once;

pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.0 == 0 {
            return vec![];
        }

        let mut rows = vec![vec![1]];
        for _ in 0..(self.0 - 1) {
            let last_row = rows.last().unwrap();
            let row = last_row.windows(2).map(|w| w.iter().sum::<u32>());
            let full_row = once(1).chain(row).chain(once(1)).collect::<Vec<_>>();
            rows.push(full_row);
        }
        rows
    }
}
