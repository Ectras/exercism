def is_vowel(c: str) -> bool:
    return c in "aeiou"


def is_consonant(c: str) -> bool:
    return not is_vowel(c)


def consonant_prefix_len(word: str) -> int:
    prefix_len = 0
    for c in word:
        if is_consonant(c):
            prefix_len += 1
        else:
            break
    return prefix_len


def move_back(word: str, letters: int) -> str:
    return word[letters:] + word[:letters]


def reorder_word(word: str) -> str:
    # Rule 1
    if is_vowel(word[0]) or word.startswith("xr") or word.startswith("yt"):
        return word

    prefix_len = consonant_prefix_len(word)

    # Rule 4
    ypos = word.find("y")
    if 0 < ypos <= prefix_len:
        return move_back(word, ypos)

    # Rule 3
    qpos = word.find("qu")
    if qpos == prefix_len - 1:
        return move_back(word, qpos + 2)

    # Rule 2
    return move_back(word, prefix_len)


def translate_word(text: str) -> str:
    return reorder_word(text) + "ay"


def translate(text: str) -> str:
    return " ".join([translate_word(word) for word in text.split()])
