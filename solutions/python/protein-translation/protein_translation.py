def get_protein(codon: str) -> str:
    if codon == "AUG":
        return "Methionine"
    if codon in ("UUU", "UUC"):
        return "Phenylalanine"
    if codon in ("UUA", "UUG"):
        return "Leucine"
    if codon in ("UCU", "UCC", "UCA", "UCG"):
        return "Serine"
    if codon in ("UAU", "UAC"):
        return "Tyrosine"
    if codon in ("UGU", "UGC"):
        return "Cysteine"
    if codon == "UGG":
        return "Tryptophan"
    if codon in ("UAA", "UAG", "UGA"):
        return "STOP"
    raise ValueError(f"Unknown codon: {codon}")


def proteins(strand: str) -> list[str]:
    assert len(strand) % 3 == 0

    out = []
    for codon_start in range(0, len(strand), 3):
        codon = strand[codon_start : codon_start + 3]
        protein = get_protein(codon)
        if protein == "STOP":
            break
        out.append(protein)
    return out
