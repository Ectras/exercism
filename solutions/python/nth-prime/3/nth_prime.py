from math import isqrt


def is_prime(number: int) -> bool:
    for divisor in range(2, isqrt(number) + 1):
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
