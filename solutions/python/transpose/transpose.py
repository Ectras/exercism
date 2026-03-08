from itertools import zip_longest


def transpose(text: str) -> str:
    lines = text.splitlines()

    # Make sure that each line is at least as long as the line below it
    for i in range(len(lines) - 2, -1, -1):
        lines[i] = lines[i].ljust(len(lines[i + 1]))

    # Use zip_longest(*) to iterate column-wise over the lines (where already ended lines return None)
    columns = zip_longest(*lines)
    return "\n".join(
        "".join(letter for letter in column if letter) for column in columns
    )
