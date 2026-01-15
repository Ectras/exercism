from random import randint


def roll_dice() -> int:
    return randint(1, 6)


def roll_score() -> int:
    rolls = [roll_dice() for _ in range(4)]
    return sum(rolls) - min(rolls)


class Character:
    def __init__(self):
        self.strength = roll_score()
        self.dexterity = roll_score()
        self.constitution = roll_score()
        self.intelligence = roll_score()
        self.wisdom = roll_score()
        self.charisma = roll_score()
        self.hitpoints = 10 + modifier(self.constitution)

    def ability(self) -> int:
        return roll_score()


def modifier(value: int) -> int:
    return (value - 10) // 2
