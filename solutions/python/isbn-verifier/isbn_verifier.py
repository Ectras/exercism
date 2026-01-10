def is_valid(isbn: str) -> bool:
    isbn = isbn.replace("-", "")
    if len(isbn) != 10:
        return False
    numbers = []
    for i, c in enumerate(isbn):
        if i == 9 and c == "X":
            number = 10
        else:
            try:
                number = int(c)
            except ValueError:
                return False
        numbers.append(number)

    checksum = sum(x * (10 - i) for i, x in enumerate(numbers))
    return checksum % 11 == 0
