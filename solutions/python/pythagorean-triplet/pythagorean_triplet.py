def triplets_with_sum(number: int) -> list[list[int]]:
    results = []
    for b in range(1, number // 2):
        denom = 2 * (number - b)
        num = number * (number - 2 * b)
        a = num // denom
        c = number - a - b
        if not (a < b < c):
            continue
        if a * a + b * b != c * c:
            continue
        results.append([a, b, c])
    return results


# Two conditions:
# (1): a + b + c = N
# (2): a^2 + b^2 = c^2

# Rewrite (1):
# (N - a - b)^2 = c^2

# Equal (1) and (2) (both are equal to c^2):
# (N - a - b)^2 = a^2 + b^2

# Simplify the expression:
# a^2 + 2ab - 2aN + b^2 - 2bN + N^2 = a^2 + b^2
# 2ab - 2aN - 2bN + N^2 = 0
# 2a(b - N) - 2bN + N^2 = 0
# a = (2bN - N^2) / (2b - 2N)
# a = (N^2 - 2bN) / (2N - 2b)
# a = N(N - 2b) / (2(N - b))
