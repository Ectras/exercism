def recursive_flatten(maybe_iterable, out: list):
    try:
        for item in maybe_iterable:
            recursive_flatten(item, out)
    except TypeError:
        # iterable is *not* an iterable, so treat it as an item
        if maybe_iterable is not None:
            out.append(maybe_iterable)


def flatten(iterable):
    out = []
    recursive_flatten(iterable, out)
    return out
