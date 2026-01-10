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


def value(colors: list[str]) -> int:
    colors = colors[:2]
    d1 = COLORS.index(colors[0])
    d2 = COLORS.index(colors[1])
    return d1 * 10 + d2
