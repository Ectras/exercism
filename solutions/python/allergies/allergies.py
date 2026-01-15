FLAGS = {
    "eggs": 1,
    "peanuts": 2,
    "shellfish": 4,
    "strawberries": 8,
    "tomatoes": 16,
    "chocolate": 32,
    "pollen": 64,
    "cats": 128,
}


class Allergies:

    def __init__(self, score: int):
        self.score = score

    def allergic_to(self, item: str) -> bool:
        flag = FLAGS[item]
        return bool(self.score & flag)

    @property
    def lst(self) -> list[str]:
        return [item for item in FLAGS if self.allergic_to(item)]
