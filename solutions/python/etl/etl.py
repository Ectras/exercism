def transform(legacy_data: dict[int, list[str]]) -> dict[str, int]:
    return {c.lower(): x for x, letters in legacy_data.items() for c in letters}
