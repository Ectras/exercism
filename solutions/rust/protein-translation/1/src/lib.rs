use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    table: HashMap<&'a str, &'a str>
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.table.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut out = Vec::new();
        for codon in rna.as_bytes().chunks(3) {
            let codon = String::from_utf8_lossy(codon);
            let name = self.name_for(&codon)?;
            if name == "stop codon" {
                break;
            }
            out.push(name);
        }

        Some(out)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { table: HashMap::from_iter(pairs) }
}
