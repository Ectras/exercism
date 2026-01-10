from typing import Callable, TypeVar

T = TypeVar("T")
U = TypeVar("U")


def append(list1: list[T], list2: list[T]) -> list[T]:
    list1 += list2
    return list1


def concat(lists: list[list[T]]) -> list[T]:
    out = []
    for list in lists:
        out += list
    return out


def filter(function: Callable[[T], bool], list: list[T]) -> list[T]:
    return [item for item in list if function(item)]


def length(list: list[T]) -> int:
    return len(list)


def map(function: Callable[[T], U], list: list[T]) -> list[U]:
    return [function(item) for item in list]


def foldl(function: Callable[[U, T], U], list: list[T], initial: U) -> U:
    acc = initial
    for item in list:
        acc = function(acc, item)
    return acc


def foldr(function: Callable[[U, T], U], list: list[T], initial: U) -> U:
    return foldl(function, list[::-1], initial)


def reverse(list: list[T]) -> list[T]:
    return list[::-1]
