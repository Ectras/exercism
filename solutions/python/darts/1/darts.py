def score(x: float, y: float) -> int:
    radius = x**2 + y**2
    if radius <= 1:
        return 10
    if radius <= 5**2:
        return 5
    if radius <= 10**2:
        return 1
    return 0
