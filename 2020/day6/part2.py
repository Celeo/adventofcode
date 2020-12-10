from collections import Counter
from itertools import chain
from typing import List


def flatten(*args: List[str]) -> List[str]:
    return list(chain.from_iterable(args))


def main() -> None:
    with open("2020/day6/input.txt") as f:
        lines = [line.strip() for line in f.readlines()]
    counts = []
    group = []

    def calc(group: List[List[str]]) -> None:
        person_count = len(group)
        ans_count = Counter(flatten(*group))
        counts.append(len([c for c in ans_count if ans_count[c] == person_count]))

    for line in lines:
        if not line:
            calc(group)
            group = []
            continue
        group.append(list(line))
    calc(group)
    print(sum(counts))


if __name__ == "__main__":
    main()
