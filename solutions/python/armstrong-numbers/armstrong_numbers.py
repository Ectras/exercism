def is_armstrong_number(number: int) -> bool:
    number_str = str(number)
    return number == sum(int(digit) ** len(number_str) for digit in number_str)
