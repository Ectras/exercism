ENDINGS = ["", "thousand", "million", "billion"]
ONES = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
]
TEN_ONES = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
]
TENS = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"]


def say_ones(number: int) -> str:
    """Says a number of 1 - 19."""
    return ONES[number]


def say_tens(number: int) -> str:
    """Says a number of 1 - 99."""
    tens, rest = divmod(number, 10)
    words = []
    if tens == 1:
        # Special cases 10..20
        return TEN_ONES[number - 10]
    if tens > 0:
        words.append(TENS[tens - 2])
    if rest > 0:
        words.append(say_ones(rest))
    return "-".join(words)


def say_hundreds(number: int, suffix: str) -> list[str]:
    """Says a number of 1 - 999."""
    if number == 0:
        return []

    hundred_digit, rest = divmod(number, 100)
    words = []
    if hundred_digit > 0:
        words.append(say_ones(hundred_digit))
        words.append("hundred")
    if rest > 0:
        words.append(say_tens(rest))
    if suffix != "":
        words.append(suffix)
    return words


def say(number: int) -> str:
    if number < 0:
        raise ValueError("input out of range")
    if number > 999_999_999_999:
        raise ValueError("input out of range")
    if number == 0:
        return "zero"

    words = []
    for i, ending in reversed(list(enumerate(ENDINGS))):
        factor = 10 ** (3 * i)
        words += say_hundreds(number // factor, ending)
        number %= factor
    return " ".join(words)
