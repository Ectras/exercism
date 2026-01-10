def find(search_list: list, value) -> int:
    if len(search_list) == 0:
        raise ValueError("value not in array")

    mid = len(search_list) // 2
    if search_list[mid] == value:
        return mid
    if search_list[mid] > value:
        return find(search_list[:mid], value)
    else:
        return mid + 1 + find(search_list[mid + 1 :], value)
