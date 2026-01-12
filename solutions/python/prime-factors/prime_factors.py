def factors(value: int) -> list[int]:
    out = []
    divisor = 2
    while value > 1:
        while value % divisor == 0:
            out.append(divisor)
            value //= divisor
        divisor += 1
    return out
