# Globals for the directions
# Change the values as you see fit
EAST = 0
NORTH = 1
WEST = 2
SOUTH = 3


def rotate_right(direction: int) -> int:
    return (direction - 1) % 4


def rotate_left(direction: int) -> int:
    return (direction + 1) % 4


def vector(direction: int) -> tuple[int, int]:
    if direction == EAST:
        return (1, 0)
    if direction == NORTH:
        return (0, 1)
    if direction == WEST:
        return (-1, 0)
    if direction == SOUTH:
        return (0, -1)
    raise ValueError(f"Unknown direction: {direction}")


class Robot:
    def __init__(self, direction: int = NORTH, x_pos: int = 0, y_pos: int = 0):
        self.direction = direction
        self.coordinates = (x_pos, y_pos)

    def move(self, instructions: str):
        for instruction in instructions:
            match instruction:
                case "L":
                    self.direction = rotate_left(self.direction)
                case "R":
                    self.direction = rotate_right(self.direction)
                case "A":
                    vec = vector(self.direction)
                    self.coordinates = (
                        self.coordinates[0] + vec[0],
                        self.coordinates[1] + vec[1],
                    )
                case _:
                    raise ValueError(f"Unknown instruction: {instruction}")
