def abbreviate(words: str) -> str:
    words = words.translate(str.maketrans({"-": " ", "'": "", "_": "", ",": ""}))
    abbr = "".join(word[0] for word in words.split())
    return abbr.upper()
