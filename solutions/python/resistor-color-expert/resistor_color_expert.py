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

TOLERANCES = {
    "grey": "0.05",
    "violet": "0.1",
    "blue": "0.25",
    "green": "0.5",
    "brown": "1",
    "red": "2",
    "gold": "5",
    "silver": "10",
}

PREFIXES = ["", "kilo", "mega", "giga"]


def format_resistance(value: float) -> tuple[str, str]:
    prefix = 0
    while value >= 1000:
        value /= 1000
        prefix += 1
    return f"{value:g}", PREFIXES[prefix]


def resistor_label(colors: list[str]) -> str:
    if colors == ["black"]:
        value = 0
        tolerance = ""
    else:
        assert 4 <= len(colors) <= 5
        digits = [COLORS.index(d) for d in colors[:-1]]
        tolerance = f" ±{TOLERANCES[colors[-1]]}%"
        expo = digits[-1]
        value = digits[0]
        for val in digits[1:-1]:
            value = (value * 10) + val
        value *= 10**expo
    number, prefix = format_resistance(value)
    return f"{number} {prefix}ohms{tolerance}"
