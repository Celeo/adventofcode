from itertools import combinations
from typing import List


def valid(numbers: List[int], index: int) -> bool:
    selection = numbers[index - 25 if index > 25 else 0 : index]
    add_to = numbers[index]
    combos = combinations(selection, 2)
    for combo in combos:
        if sum(combo) == add_to and combo[0] != combo[1]:
            return True
    return False


def main() -> None:
    with open("2020/day9/input.txt") as f:
        numbers = [int(line.strip()) for line in f.readlines()]
    index = 25
    while True:
        if not valid(numbers, index):
            print(numbers[index])
            break
        index += 1


if __name__ == "__main__":
    main()
