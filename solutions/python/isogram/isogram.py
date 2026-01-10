def is_isogram(text: str) -> bool:
    letters = [c.lower() for c in text if c.isalpha()]
    return len(set(letters)) == len(letters)
