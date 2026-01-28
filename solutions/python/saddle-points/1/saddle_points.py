def saddle_points(matrix: list[list[int]]) -> list[dict[str, int]]:
    if len(matrix) == 0:
        return []

    tallest_per_row = []
    for row in matrix:
        if len(row) != len(matrix[0]):
            raise ValueError("irregular matrix")
        tallest_per_row.append(max(row))
    shortest_per_col = [min(row[c] for row in matrix) for c in range(len(matrix[0]))]

    results = []
    for r, row in enumerate(matrix):
        for c, height in enumerate(row):
            if height == tallest_per_row[r] == shortest_per_col[c]:
                results.append({"row": r + 1, "column": c + 1})
    return results
