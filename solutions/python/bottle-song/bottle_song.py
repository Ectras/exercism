COUNTS = [
    "no",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
]


def bottle(count: int) -> str:
    return "bottle" + ("s" if count != 1 else "")


def verse(bottles: int) -> list[str]:
    first = f"{COUNTS[bottles].title()} green {bottle(bottles)} hanging on the wall,"
    second = "And if one green bottle should accidentally fall,"
    third = f"There'll be {COUNTS[bottles - 1]} green {bottle(bottles - 1)} hanging on the wall."
    return [first, first, second, third]


def recite(start: int, take: int = 1) -> list[str]:
    out = []
    for s in range(start, start - take, -1):
        if s < start:
            out.append("")
        out += verse(s)
    return out
