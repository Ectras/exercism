def roman(number: int) -> str:
    def divmod(x: int, y: int) -> tuple[int, int]:
        return x // y, x % y

    out = ""
    thousands, number = divmod(number, 1000)
    out += thousands * "M"

    # number is now < 1000
    if number >= 900:
        number -= 900
        out += "CM"
    elif number >= 500:
        number -= 500
        out += "D"
    elif number >= 400:
        number -= 400
        out += "CD"
    hundreds, number = divmod(number, 100)
    out += hundreds * "C"

    # number is now < 100
    if number >= 90:
        number -= 90
        out += "XC"
    elif number >= 50:
        number -= 50
        out += "L"
    elif number >= 40:
        number -= 40
        out += "XL"
    tens, number = divmod(number, 10)
    out += tens * "X"

    # number is now < 10
    if number == 9:
        number -= 9
        out += "IX"
    elif number >= 5:
        number -= 5
        out += "V"
    elif number == 4:
        number -= 4
        out += "IV"
    out += number * "I"

    return out
