def rows(letter: str) -> list[str]:
    letters = ord(letter) - ord("A") + 1
    width = 2 * letters - 1
    mid = letters - 1
    out = []

    # Only do upper half of the diamond explicitly
    for row in range(letters):
        row_letter = chr(row + ord("A"))
        # Also works for the first row, as left_pos == right_pos == mid
        left_pos = mid - row
        right_pos = mid + row
        chars = [
            row_letter if i in (left_pos, right_pos) else " " for i in range(width)
        ]
        out.append("".join(chars))

    # Mirror the upper half down
    out += reversed(out[:-1])
    return out
