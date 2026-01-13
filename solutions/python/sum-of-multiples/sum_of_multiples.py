def sum_of_multiples(limit: int, multiples: list[int]) -> int:
    combined_multiples = set()
    for multiple in multiples:
        if multiple == 0:
            continue

        for j in range(1, (limit - 1) // multiple + 1):
            combined_multiples.add(j * multiple)

    return sum(combined_multiples)
