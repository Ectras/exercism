def saddle_points(matrix: list[list[int]]) -> list[dict[str, int]]:
    if any(len(row) != len(matrix[0]) for row in matrix[1:]):
        raise ValueError("irregular matrix")

    tallest_per_row = [max(row) for row in matrix]
    shortest_per_col = [min(col) for col in zip(*matrix)]

    results = []
    for r, row in enumerate(matrix):
        for c, height in enumerate(row):
            if height == tallest_per_row[r] == shortest_per_col[c]:
                results.append({"row": r + 1, "column": c + 1})
    return results
