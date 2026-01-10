def decode(string: str) -> str:
    counter = 0
    out = []
    for c in string:
        if c.isdecimal():
            counter = counter * 10 + int(c)
        else:
            out.append(max(1, counter) * c)
            counter = 0
    return "".join(out)


def encode(string: str) -> str:
    counter = 1
    out = []
    for i, c in enumerate(string):
        if i == len(string) - 1 or c != string[i + 1]:
            # End current run
            text = c
            if counter > 1:
                text = str(counter) + text
            out.append(text)
            counter = 1
        else:
            counter += 1
    return "".join(out)
