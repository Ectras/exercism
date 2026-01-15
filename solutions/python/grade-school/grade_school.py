from collections import defaultdict


class School:
    def __init__(self):
        self.grades = defaultdict(set)
        self.student_added = []

    def add_student(self, name: str, grade: int):
        can_be_added = all(name not in students for students in self.grades.values())
        if can_be_added:
            self.grades[grade].add(name)
        self.student_added.append(can_be_added)

    def roster(self) -> list[str]:
        out = []
        for grade_number in sorted(self.grades):
            out += self.grade(grade_number)
        return out

    def grade(self, grade_number: int) -> list[str]:
        return sorted(self.grades[grade_number])

    def added(self) -> list[bool]:
        return self.student_added
