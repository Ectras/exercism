def isqrt(y: int) -> int:
    """Integer square root (binary search).
    Taken from https://en.wikipedia.org/wiki/Integer_square_root"""
    L = 0  # lower bound of the square root
    R = y + 1  # upper bound of the square root

    while L != R - 1:
        M = (L + R) // 2  # midpoint to test
        if M * M <= y:
            L = M
        else:
            R = M
    return L


def is_prime(number: int) -> bool:
    if number == 2:
        return True
    if number % 2 == 0:  # Fast check
        return False
    for divisor in range(3, isqrt(number) + 1):
        if number % divisor == 0:
            return False
    return True


def prime(number: int) -> int:
    if number == 0:
        raise ValueError("there is no zeroth prime")

    primes_found = 0
    candidate = 1
    while primes_found < number:
        candidate += 1
        while not is_prime(candidate):
            candidate += 1
        primes_found += 1
    return candidate
