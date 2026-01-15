from random import randint


class Character:
    def __init__(self):
        self.strength = self.ability()
        self.dexterity = self.ability()
        self.constitution = self.ability()
        self.intelligence = self.ability()
        self.wisdom = self.ability()
        self.charisma = self.ability()
        self.hitpoints = 10 + modifier(self.constitution)

    def ability(self) -> int:
        rolls = [randint(1, 6) for _ in range(4)]
        return sum(rolls) - min(rolls)


def modifier(value: int) -> int:
    return (value - 10) // 2
