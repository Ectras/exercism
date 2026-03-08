pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();

    // Find the minimum element in each column
    let column_count = input.get(0).map(|row| row.len()).unwrap_or_default();
    let mut min_per_column = vec![u64::MAX; column_count];
    for row in input {
        for (i, &elem) in row.iter().enumerate() {
            min_per_column[i] = elem.min(min_per_column[i]);
        }
    }

    // Find saddle points
    for (i, row) in input.iter().enumerate() {
        if row.is_empty() {
            continue;
        }

        let max_in_row = row.iter().max().copied().unwrap();
        for (j, &elem) in row.iter().enumerate() {
            if elem == min_per_column[j] && elem == max_in_row {
                out.push((i, j))
            }
        }
    }
    
    out
}
