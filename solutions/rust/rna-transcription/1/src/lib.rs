#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DNA {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
}

impl DNA {
    fn rna_complement(&self) -> RNA {
        match self {
            DNA::Guanine => RNA::Cytosine,
            DNA::Cytosine => RNA::Guanine,
            DNA::Thymine => RNA::Adenine,
            DNA::Adenine => RNA::Uracil,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RNA {
    Adenine,
    Cytosine,
    Guanine,
    Uracil,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: Vec<DNA>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: Vec<RNA>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let sequence = dna
            .char_indices()
            .map(|(i, c)| match c {
                'A' => Ok(DNA::Adenine),
                'C' => Ok(DNA::Cytosine),
                'G' => Ok(DNA::Guanine),
                'T' => Ok(DNA::Thymine),
                _ => Err(i),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { sequence })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            sequence: self
                .sequence
                .into_iter()
                .map(|nuc| nuc.rna_complement())
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let sequence = rna
            .char_indices()
            .map(|(i, c)| match c {
                'A' => Ok(RNA::Adenine),
                'C' => Ok(RNA::Cytosine),
                'G' => Ok(RNA::Guanine),
                'U' => Ok(RNA::Uracil),
                _ => Err(i),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { sequence })
    }
}
