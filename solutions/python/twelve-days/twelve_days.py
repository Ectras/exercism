DAYS = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
]

THINGS = [
    "a Partridge in a Pear Tree",
    "two Turtle Doves",
    "three French Hens",
    "four Calling Birds",
    "five Gold Rings",
    "six Geese-a-Laying",
    "seven Swans-a-Swimming",
    "eight Maids-a-Milking",
    "nine Ladies Dancing",
    "ten Lords-a-Leaping",
    "eleven Pipers Piping",
    "twelve Drummers Drumming",
]


def comma_join(parts: list[str]) -> str:
    out = []
    for i, part in enumerate(parts):
        if i > 0:
            if i == len(parts) - 1:
                conj = ", and "
            else:
                conj = ", "
            out.append(conj)
        out.append(part)
    return "".join(out)


def recite_single(verse: int) -> str:
    prefix = f"On the {DAYS[verse]} day of Christmas my true love gave to me"
    main = comma_join(THINGS[: verse + 1][::-1])
    return f"{prefix}: {main}."


def recite(start_verse: int, end_verse: int) -> list[str]:
    return [recite_single(verse) for verse in range(start_verse - 1, end_verse)]
