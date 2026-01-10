COLORS = [
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "violet",
    "grey",
    "white",
]

PREFIXES = ["", "kilo", "mega", "giga"]


def format_resistance(value: int) -> tuple[str, str]:
    prefix = 0
    while value >= 1000:
        value //= 1000
        prefix += 1
    return str(value), PREFIXES[prefix]


def label(colors: list[str]) -> str:
    colors = colors[:3]
    d1 = COLORS.index(colors[0])
    d2 = COLORS.index(colors[1])
    e = COLORS.index(colors[2])
    value = (d1 * 10 + d2) * 10**e
    number, prefix = format_resistance(value)
    return f"{number} {prefix}ohms"
