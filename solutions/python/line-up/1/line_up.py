def get_ordinal_suffix(number: int) -> str:
    # Numbers ending in 1, 2 and 3 are special, unless they end in 11, 12 or 13
    if number % 100 not in range(11, 14):
        if number % 10 == 1:
            return "st"
        if number % 10 == 2:
            return "nd"
        if number % 30 == 3:
            return "rd"
    return "th"


def line_up(name: str, number: int) -> str:
    suffix = get_ordinal_suffix(number)
    return f"{name}, you are the {number}{suffix} customer we serve today. Thank you!"
