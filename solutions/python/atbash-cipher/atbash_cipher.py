def flip(char: str) -> str:
    base = ord("a")
    return chr(2 * base + 25 - ord(char))


def encode(plain_text: str) -> str:
    out = []
    for c in plain_text:
        if c.isalpha():
            out.append(flip(c.lower()))
        elif c.isdecimal():
            out.append(c)

    GROUP_SIZE = 5
    groups = []
    for group_start in range(0, len(out), GROUP_SIZE):
        group = out[group_start : group_start + GROUP_SIZE]
        groups.append("".join(group))
    return " ".join(groups)


def decode(ciphered_text: str):
    out = []
    for c in ciphered_text:
        if c.isalpha():
            out.append(flip(c))
        elif c.isdecimal():
            out.append(c)
    return "".join(out)
