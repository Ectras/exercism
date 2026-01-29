COUNTS = [
    "No",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
]


def bottles(count: int) -> str:
    return f"{COUNTS[count]} green bottle" + ("s" if count != 1 else "")


def recite(start: int, take: int = 1) -> list[str]:
    out = []
    for s in range(start, start - take, -1):
        if s < start:
            out.append("")
        out.append(f"{bottles(s)} hanging on the wall,")
        out.append(f"{bottles(s)} hanging on the wall,")
        out.append("And if one green bottle should accidentally fall,")
        out.append(f"There'll be {bottles(s - 1).lower()} hanging on the wall.")
    return out
