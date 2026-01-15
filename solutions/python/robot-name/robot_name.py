import random
import string


def random_name() -> str:
    letters = random.choices(string.ascii_uppercase, k=2)
    digits = [str(random.randint(0, 9)) for _ in range(3)]
    return "".join(letters + digits)


class Robot:
    used_names = set()

    def __init__(self):
        self.name = ""
        self.reset()

    def reset(self):
        name = ""
        while True:
            name = random_name()
            if name not in Robot.used_names:
                Robot.used_names.add(name)
                self.name = name
                return
