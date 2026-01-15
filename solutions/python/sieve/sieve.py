def primes(limit: int) -> list[int]:
    factors = []
    marked = [False] * (limit + 1)
    for p in range(2, len(marked)):
        if not marked[p]:
            factors.append(p)
            for pm in range(p, len(marked), p):
                marked[pm] = True
    return factors
