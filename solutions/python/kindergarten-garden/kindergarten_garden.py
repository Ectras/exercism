from enum import Enum


class Plant(Enum):
    Grass = "G"
    Clover = "C"
    Radishes = "R"
    Violets = "V"


class Garden:
    def __init__(self, diagram: str, students: list | None = None):
        self.plant_rows = [
            [Plant(letter) for letter in line] for line in diagram.splitlines()
        ]
        self.students = (
            sorted(students)
            if students is not None
            else [
                "Alice",
                "Bob",
                "Charlie",
                "David",
                "Eve",
                "Fred",
                "Ginny",
                "Harriet",
                "Ileana",
                "Joseph",
                "Kincaid",
                "Larry",
            ]
        )

    def plants(self, student: str) -> list[str]:
        idx = self.students.index(student)
        plants = [
            self.plant_rows[r][2 * idx + i].name
            for r in range(len(self.plant_rows))
            for i in range(2)
        ]
        return plants
