def tick(matrix: list[list[int]]) -> list[list[int]]:
    DEAD = 0
    ALIVE = 1

    def get(x: int, y: int) -> int:
        if 0 <= y < len(matrix) and 0 <= x < len(matrix[y]):
            return matrix[y][x]
        return DEAD

    def alive_neighbors(x: int, y: int) -> int:
        return sum(
            get(x + i, y + j)
            for j in range(-1, 2)
            for i in range(-1, 2)
            if not (i == j == 0)
        )

    def get_new_state(old: int, x: int, y: int) -> int:
        neighbors = alive_neighbors(x, y)
        if old == ALIVE and 2 <= neighbors <= 3:
            return ALIVE
        if old == DEAD and neighbors == 3:
            return ALIVE
        return DEAD

    return [
        [get_new_state(matrix[y][x], x, y) for x in range(len(matrix[y]))]
        for y in range(len(matrix))
    ]
