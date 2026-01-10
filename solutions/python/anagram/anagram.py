def is_anagram(word1: str, word2: str) -> bool:
    lower1 = word1.lower()
    lower2 = word2.lower()
    if lower1 == lower2:
        # A word is not an anagram of itself
        return False
    return sorted(lower1) == sorted(lower2)


def find_anagrams(word: str, candidates: list[str]) -> list[str]:
    return [candidate for candidate in candidates if is_anagram(word, candidate)]
