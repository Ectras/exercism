def is_triangle(sides: list[int]) -> bool:
    return (
        len(sides) == 3
        and all(side > 0 for side in sides)
        and all(
            sides[i] + sides[j] >= sides[k]
            for i, j, k in [(0, 1, 2), (0, 2, 1), (1, 2, 0)]
        )
    )


def equilateral(sides: list[int]) -> bool:
    return is_triangle(sides) and len(set(sides)) == 1


def isosceles(sides: list[int]):
    return is_triangle(sides) and len(set(sides)) <= 2


def scalene(sides):
    return is_triangle(sides) and len(set(sides)) == 3
