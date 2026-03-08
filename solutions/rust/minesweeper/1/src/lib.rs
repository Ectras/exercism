pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut out = Vec::with_capacity(minefield.len());
    for row in 0..minefield.len() {
        let mut current_row = String::with_capacity(minefield[row].len());
        let prev_row = row.checked_sub(1).unwrap_or_default();
        let next_row = (row + 2).min(minefield.len());
        for col in 0..minefield[row].len() {
            if minefield[row].bytes().nth(col).unwrap() == b'*' {
                current_row.push('*');
                continue;
            }
            let prev_col = col.checked_sub(1).unwrap_or_default();
            let next_col = (col + 2).min(minefield[row].len());

            let mut bombs = 0;
            for j in prev_row..next_row {
                for i in prev_col..next_col {
                    if j == row && i == col {
                        continue;
                    }

                    if minefield[j].bytes().nth(i).unwrap() == b'*' {
                        bombs += 1;
                    }
                }
            }

            if bombs == 0 {
                current_row.push(' ');
            } else {
                current_row.push_str(&u32::to_string(&bombs));
            }
        }
        out.push(current_row);
    }
    out
}
