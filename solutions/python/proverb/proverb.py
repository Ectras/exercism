def proverb(*inputs: str, qualifier: str | None = None) -> list[str]:
    if len(inputs) == 0:
        return []

    lines = [
        f"For want of a {noun1} the {noun2} was lost."
        for noun1, noun2 in zip(inputs, inputs[1:])
    ]
    qualifier = f"{qualifier} " if qualifier else ""
    last = f"And all for the want of a {qualifier}{inputs[0]}."
    lines.append(last)
    return lines
