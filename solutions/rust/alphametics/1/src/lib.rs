use std::collections::{HashMap, HashSet};

fn parse_lines(input: &str) -> Vec<&str> {
    input
        .split_terminator(" == ")
        .flat_map(|t| t.split_terminator(" + "))
        .collect()
}

fn get_non_zero(lines: &[&str]) -> HashSet<char> {
    lines
        .iter()
        .filter_map(|line| {
            if line.len() > 1 {
                Some(line.chars().next().unwrap())
            } else {
                None
            }
        })
        .collect()
}

fn to_columns(lines: &[&str]) -> Vec<Vec<char>> {
    let mut out = Vec::new();
    for line in lines {
        // Add enough columns for this line
        for _ in out.len()..line.len() {
            out.push(Vec::new());
        }

        // Write the chars to the columns
        for (c, col) in line.chars().rev().zip(out.iter_mut()) {
            col.push(c);
        }
    }
    out
}

fn is_assignable(ch: char, val: u8, nnz: &HashSet<char>, mapping: &HashMap<char, u8>) -> bool {
    if val == 0 && nnz.contains(&ch) {
        return false;
    }
    mapping.values().all(|&x| x != val)
}

fn mapping_with(mapping: &HashMap<char, u8>, ch: char, val: u8) -> HashMap<char, u8> {
    let mut new = mapping.clone();
    new.insert(ch, val);
    new
}

fn try_solve(columns: &[Vec<char>], result: &str, nnz: &HashSet<char>, mut mapping: HashMap<char, u8>) -> Option<HashMap<char, u8>> {
    let mut carry = 0;
    for (i, r) in result.chars().rev().enumerate() {
        let mut sum: u32 = carry;

        // Still some column values
        if i < columns.len() {
            let col = &columns[i];
            for &c in col {
                // Is the char already assigned a value?
                if let Some(&val) = mapping.get(&c) {
                    sum += val as u32;
                } else {
                    // Try different values
                    for v in (0..=9).filter(|&v| is_assignable(c, v, nnz, &mapping)) {
                        if let Some(working_mapping) = try_solve(columns, result, nnz, mapping_with(&mapping, c, v)) {
                            return Some(working_mapping);
                        }
                    }

                    return None;
                }
            }
        }

        // Carry over
        carry = sum / 10;
        sum %= 10; // sum now fits in a u8
        
        // Check result
        if let Some(&val) = mapping.get(&r) {
            // Given current mapping, the column does not add up to the result
            if val as u32 != sum {
                return None;
            }
        } else {
            // Check if we could assign the value
            if !is_assignable(r, sum as u8, nnz, &mapping) {
                return None;
            }

            // If yes, we have to assign it
            mapping = mapping_with(&mapping, r, sum as u8);
        }
        
    }
    Some(mapping)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let rows = parse_lines(input);
    let nnz = get_non_zero(&rows);
    let cols = to_columns(&rows[..rows.len() - 1]);
    let result = rows[rows.len() - 1];
    if cols.len() > result.len() {
        return None;
    }
    try_solve(&cols, result, &nnz, HashMap::new())
}
