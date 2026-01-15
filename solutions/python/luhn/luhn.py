class Luhn:
    def __init__(self, card_num: str):
        self.card_num = card_num

    def valid(self) -> bool:
        # Remove spaces
        card_num = self.card_num.replace(" ", "")

        # Check that it is a number
        if len(card_num) <= 1 or not card_num.isdecimal():
            return False

        # Process number
        numbers = [int(digit) for digit in card_num]
        for i in range(-2, -(len(numbers) + 1), -2):
            val = 2 * numbers[i]
            if val > 9:
                val -= 9
            numbers[i] = val

        return sum(numbers) % 10 == 0
