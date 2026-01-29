from typing import Any, Sequence, TypeVar


T = TypeVar("T")

NUMBERS = [
    (" _ ", "| |", "|_|", "   "),
    ("   ", "  |", "  |", "   "),
    (" _ ", " _|", "|_ ", "   "),
    (" _ ", " _|", " _|", "   "),
    ("   ", "|_|", "  |", "   "),
    (" _ ", "|_ ", " _|", "   "),
    (" _ ", "|_ ", "|_|", "   "),
    (" _ ", "  |", "  |", "   "),
    (" _ ", "|_|", "|_|", "   "),
    (" _ ", "|_|", " _|", "   "),
]


def chunks(lst: Sequence[T], n: int) -> list[Any]:
    assert len(lst) % n == 0
    return [lst[i : i + n] for i in range(0, len(lst), n)]


def convert_number(input_grid: tuple[str, str, str, str]) -> str:
    try:
        return str(NUMBERS.index(input_grid))
    except ValueError:
        return "?"


def convert_row(input_grid: list[str]) -> str:
    numbers = [chunks(row, 3) for row in input_grid]
    return "".join(convert_number(number) for number in zip(*numbers))


def convert(input_grid: list[str]) -> str:
    if len(input_grid) % 4 != 0:
        raise ValueError("Number of input lines is not a multiple of four")
    line_lengths = set(len(line) for line in input_grid)
    if len(line_lengths) > 1 or line_lengths.pop() % 3 != 0:
        raise ValueError("Number of input columns is not a multiple of three")

    return ",".join(convert_row(row) for row in chunks(input_grid, 4))
