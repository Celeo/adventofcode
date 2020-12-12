def valid(n: int, allow_more_than_two: bool = True) -> bool:
    s = str(n)
    if len(s) != 6:
        return False
    if s[0] > s[1] or s[1] > s[2] or s[2] > s[3] or s[3] > s[4] or s[4] > s[5]:
        return False
    found = False
    for i in range(0, 10):
        if allow_more_than_two:
            if f"{i}{i}" in s:
                found = True
                break
        else:
            if f"{i}{i}" in s and f"{i}{i}{i}" not in s:
                found = True
                break
    return found


def main() -> None:
    with open("2019/day4/input.txt") as f:
        range_low, range_high = [int(p) for p in f.readline().split("-")]
    valid_part_1 = len([e for e in range(range_low, range_high + 1) if valid(e)])
    print(valid_part_1)
    valid_part_2 = len([e for e in range(range_low, range_high + 1) if valid(e, False)])
    print(valid_part_2)


if __name__ == "__main__":
    main()
