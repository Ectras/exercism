def decode(string: str) -> str:
    i = 0
    out = []
    while i < len(string):
        number_end = i
        while number_end < len(string) and string[number_end].isdecimal():
            number_end += 1

        if number_end == i:
            # There was no number
            out.append(string[i])
            i += 1
        else:
            # There was a number, read the next char, add it and then skip the whole thing
            number = int(string[i:number_end])
            data = string[number_end]
            out.append(data * number)
            i += number_end - i + 1
    return "".join(out)


def encode(string: str) -> str:
    def write_counter(out: list[str], counter: int, char: str):
        if counter > 1:
            out.append(f"{counter}{char}")
        elif counter == 1:
            out.append(char)

    last_char = "?"
    counter = 0
    out = []
    for c in string:
        if c == last_char:
            counter += 1
        else:
            write_counter(out, counter, last_char)
            last_char = c
            counter = 1
    write_counter(out, counter, last_char)
    return "".join(out)
