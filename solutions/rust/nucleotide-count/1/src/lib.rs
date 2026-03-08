use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .get(&nucleotide)
        .ok_or(nucleotide)
        .copied()
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut out = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for char in dna.chars() {
        match char {
            'A' | 'C' | 'G' | 'T' => out.entry(char).and_modify(|c| *c += 1),
            _ => return Err(char),
        };
    }
    Ok(out)
}
