def square_root(number: int):
    for i in range(1, 2 + number // 2):
        if i * i == number:
            return i
    raise ValueError(f"Can't compute square root of {number}")
