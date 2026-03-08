pub struct Matrix {
    /// The matrix data in row-major storage.
    data: Vec<u32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let data = input
            .split('\n')
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let rows = data.len();
        let cols = data.first().unwrap().len();

        let flat_data = data.into_iter().flatten().collect();

        Self {
            data: flat_data,
            rows,
            cols,
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if 1 <= row_no && row_no <= self.rows {
            Some(
                self.data
                    .iter()
                    .skip((row_no - 1) * self.cols)
                    .take(self.cols)
                    .copied()
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if 1 <= col_no && col_no <= self.cols {
            Some(
                self.data
                    .iter()
                    .skip(col_no - 1)
                    .step_by(self.cols)
                    .copied()
                    .collect(),
            )
        } else {
            None
        }
    }
}
