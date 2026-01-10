def slices(series: str, length: int) -> list[str]:
    if length == 0:
        raise ValueError("slice length cannot be zero")
    if length < 0:
        raise ValueError("slice length cannot be negative")
    if series == "":
        raise ValueError("series cannot be empty")
    if length > len(series):
        raise ValueError("slice length cannot be greater than series length")

    return [series[start : start + length] for start in range(len(series) - length + 1)]
