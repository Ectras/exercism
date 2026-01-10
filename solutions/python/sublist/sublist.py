"""
This exercise stub and the test suite contain several enumerated constants.

Enumerated constants can be done with a NAME assigned to an arbitrary,
but unique value. An integer is traditionally used because it’s memory
efficient.
It is a common practice to export both constants and functions that work with
those constants (ex. the constants in the os, subprocess and re modules).

You can learn more here: https://en.wikipedia.org/wiki/Enumerated_type
"""

# Possible sublist categories.
# Change the values as you see fit.
SUBLIST = 0b10
SUPERLIST = 0b01
EQUAL = 0b11
UNEQUAL = 0b00


def is_sublist(smaller: list, longer: list) -> bool:
    if len(longer) < len(smaller):
        return False

    for i in range(len(longer) - len(smaller) + 1):
        if smaller == longer[i : i + len(smaller)]:
            return True
    return False


def sublist(list_one: list, list_two: list):
    result = UNEQUAL
    if is_sublist(list_one, list_two):
        result |= SUBLIST
    if is_sublist(list_two, list_one):
        result |= SUPERLIST
    return result
