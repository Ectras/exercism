from collections import Counter


def count_words(sentence: str) -> dict[str, int]:
    filtered = []
    for i, c in enumerate(sentence):
        if c.isalnum() or c.isspace():
            filtered.append(c.lower())
        elif (
            c == "'"
            and (0 < i < len(sentence) - 1)
            and sentence[i - 1].isalpha()
            and sentence[i + 1].isalpha()
        ):
            # Keep "protected" apostrophes
            filtered.append(c)
        else:
            # Replace special chars by space such that e.g. "a,b,c" is split correctly
            filtered.append(" ")
    final = "".join(filtered)
    words = final.split()
    return dict(Counter(words))
