def factors(number: int) -> list[int]:
    facs = [1]
    for i in range(2, 1 + number // 2):
        if number % i == 0:
            facs.append(i)
    return facs


def classify(number: int):
    """A perfect number equals the sum of its positive divisors.

    :param number: int a positive integer
    :return: str the classification of the input integer
    """
    if number < 1:
        raise ValueError("Classification is only possible for positive integers.")

    aliquot_sum = sum(factors(number))
    if number > aliquot_sum or aliquot_sum == 1:
        return "deficient"
    if number < aliquot_sum:
        return "abundant"
    return "perfect"
