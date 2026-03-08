pub struct Matrix {
    /// The matrix data in row-major storage.
    data: Vec<u32>,
    cols: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows = input
            .split('\n')
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let cols = rows.first().unwrap().len();

        let flat_data = rows.into_iter().flatten().collect();

        Self {
            data: flat_data,
            cols,
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        let end = row_no * self.cols;
        if 0 < row_no && end <= self.data.len() {
            let start = (row_no - 1) * self.cols;
            Some(self.data[start..end].to_owned())
        } else {
            None
        }
    }

    pub fn column(&self, mut col_no: usize) -> Option<Vec<u32>> {
        if 0 < col_no && col_no <= self.cols {
            col_no -= 1; // for 0-based indexing
            Some(
                self.data
                    .iter()
                    .skip(col_no)
                    .step_by(self.cols)
                    .copied()
                    .collect(),
            )
        } else {
            None
        }
    }
}
