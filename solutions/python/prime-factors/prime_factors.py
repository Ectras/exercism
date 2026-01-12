from math import isqrt


def get_prime_numbers(value: int) -> list[int]:
    # Sieve of Eratosthenes
    marked = [False for _ in range(value + 1)]
    primes = []
    for i in range(2, len(marked)):
        if marked[i]:
            continue
        primes.append(i)
        for j in range(1, value // i + 1):
            marked[i * j] = True
    return primes


def factors(value: int) -> list[int]:
    out = []
    for p in get_prime_numbers(isqrt(value)):
        while value % p == 0:
            value //= p
            out.append(p)
        if value == 1:
            break
    # if the remainder is > 1, then it is a prime factor itself
    if value > 1:
        out.append(value)
    return out
