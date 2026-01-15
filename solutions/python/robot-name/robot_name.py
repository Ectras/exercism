import random
import string


def random_name() -> str:
    letters = random.choices(string.ascii_uppercase, k=2)
    digits = random.choices(string.digits, k=3)
    return "".join(letters + digits)


class Robot:
    used_names = set()

    def __init__(self):
        self.name = Robot.find_next_name()

    def reset(self):
        self.name = Robot.find_next_name()

    @staticmethod
    def find_next_name() -> str:
        while True:
            name = random_name()
            if name not in Robot.used_names:
                Robot.used_names.add(name)
                return name
