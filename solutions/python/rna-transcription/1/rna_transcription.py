def to_rna(dna_strand: str) -> str:
    return dna_strand.translate(
        {ord("G"): "C", ord("C"): "G", ord("T"): "A", ord("A"): "U"}
    )
