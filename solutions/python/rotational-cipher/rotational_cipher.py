def rotate_single(char: str, key: int, is_lower: bool) -> str:
    base = ord("a") if is_lower else ord("A")
    value = ord(char) - base
    rotated_value = (value + key) % 26
    return chr(rotated_value + base)


def rotate(text: str, key: int) -> str:
    out = []
    for c in text:
        if c.isalpha():
            c = rotate_single(c, key, c.islower())
        out.append(c)
    return "".join(out)
