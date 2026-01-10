def pop_token(tokens: list[str]) -> str:
    if len(tokens) == 0:
        raise ValueError("syntax error")
    return tokens.pop()


def pop_expect(tokens: list[str], expected: str):
    if pop_token(tokens) != expected:
        raise ValueError("syntax error")


def pop_int(tokens: list[str]) -> int:
    text = pop_token(tokens)
    try:
        return int(text)
    except ValueError:
        raise ValueError("syntax error")


def is_int(token: str) -> bool:
    try:
        int(token)
        return True
    except ValueError:
        return False


def answer(question: str) -> int:
    tokens = question.removesuffix("?").split()
    tokens.reverse()

    pop_expect(tokens, "What")
    pop_expect(tokens, "is")

    result = pop_int(tokens)
    while len(tokens) > 0:
        operation = pop_token(tokens)
        if operation == "plus":
            result += pop_int(tokens)
        elif operation == "minus":
            result -= pop_int(tokens)
        elif operation == "multiplied":
            pop_expect(tokens, "by")
            result *= pop_int(tokens)
        elif operation == "divided":
            pop_expect(tokens, "by")
            result //= pop_int(tokens)
        else:
            if is_int(operation):
                raise ValueError("syntax error")
            else:
                raise ValueError("unknown operation")
    return result
