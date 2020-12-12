from typing import Tuple


def count_diff(a: str, b: str) -> Tuple[int, str]:
    c, r = 0, ""
    for i in range(len(a)):
        if a[i] != b[i]:
            c += 1
        else:
            r += a[i]
    return (c, r)


def main() -> None:
    with open("2018/day2/input.txt") as f:
        ids = [l.strip() for l in f.readlines()]
    for a in ids:
        for b in ids:
            if a == b:
                continue
            diff = count_diff(a, b)
            if diff[0] == 1:
                print(a, b, diff[1])
                return


if __name__ == "__main__":
    main()
