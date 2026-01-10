def commands(binary_str: str) -> list[str]:
    out = []
    for i, digit in enumerate(reversed(binary_str)):
        if digit == "1":
            if i == 0:
                out += ["wink"]
            elif i == 1:
                out += ["double blink"]
            elif i == 2:
                out += ["close your eyes"]
            elif i == 3:
                out += ["jump"]
            elif i == 4:
                out.reverse()
            else:
                raise ValueError("Unknown command")
        else:
            assert digit == "0"
    return out
