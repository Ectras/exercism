def sum_of_multiples(limit: int, multiples: list[int]) -> int:
    combined_multiples = {
        i
        for multiple in multiples
        if multiple != 0
        for i in range(multiple, limit, multiple)
    }
    return sum(combined_multiples)
