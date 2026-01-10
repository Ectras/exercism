from collections import defaultdict


def get_opening_bracket(closing_bracket: str) -> str:
    if closing_bracket == ")":
        return "("
    if closing_bracket == "]":
        return "["
    if closing_bracket == "}":
        return "{"
    raise ValueError(f"Can't find opening bracket to {closing_bracket}")


def is_paired(input_string: str):
    counters = defaultdict(lambda: 0)
    open_brackets = []
    for char in input_string:
        if char in "([{":
            counters[char] += 1
            open_brackets.append(char)
        elif char in ")]}":
            opening = get_opening_bracket(char)
            counters[opening] -= 1
            if counters[char] < 0:
                return False
            if len(open_brackets) == 0 or open_brackets.pop() != opening:
                return False
    return all(count == 0 for count in counters.values())
