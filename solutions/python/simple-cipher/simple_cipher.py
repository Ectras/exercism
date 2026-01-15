import random
import string
from itertools import cycle


def random_key() -> str:
    return "".join(random.choices(string.ascii_lowercase, k=100))


class Cipher:
    def __init__(self, key: str | None = None):
        self.key = key if key is not None else random_key()

    @staticmethod
    def _char_value(char: str) -> int:
        return ord(char) - ord("a")

    @staticmethod
    def _rotate_lowercase_char(char: str, shift: int) -> str:
        number = Cipher._char_value(char)
        shifted = (number + shift) % len(string.ascii_lowercase)
        return chr(shifted + ord("a"))

    def _transform(self, text: str, forward: bool) -> str:
        shifts = (Cipher._char_value(k) for k in cycle(self.key))
        factor = 1 if forward else -1
        return "".join(
            Cipher._rotate_lowercase_char(char, factor * shift)
            for char, shift in zip(text, shifts)
        )

    def encode(self, text: str) -> str:
        return self._transform(text, True)

    def decode(self, text: str):
        return self._transform(text, False)
