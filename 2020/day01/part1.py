from itertools import combinations
from typing import List, Optional


def get_prompt() -> List[int]:
    with open("2020/day01/input.txt") as f:
        return [int(line) for line in f.readlines()]


def main(prompt: List[int]) -> Optional[int]:
    data = combinations(prompt, 2)
    for pair in data:
        if pair[0] + pair[1] == 2020:
            return pair[0] * pair[1]


if __name__ == "__main__":
    print(main(get_prompt()))
