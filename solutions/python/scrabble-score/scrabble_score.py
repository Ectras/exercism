def score(word: str) -> int:
    def single_score(char: str) -> int:
        if char in "AEIOULNRST":
            return 1
        if char in "DG":
            return 2
        if char in "BCMP":
            return 3
        if char in "FHVWY":
            return 4
        if char == "K":
            return 5
        if char in "JX":
            return 8
        return 10

    return sum(single_score(c) for c in word.upper())
