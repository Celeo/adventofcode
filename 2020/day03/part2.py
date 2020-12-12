from functools import reduce
from typing import List


def make_map(row_inc: int, column_inc: int) -> List[str]:
    with open("2020/day3/input.txt") as f:
        data = [line.strip() for line in f.readlines()]
    height = len(data)
    starting_width = len(data[0])
    needed_width = (height - row_inc) * column_inc + (column_inc - 1)
    expand_count = needed_width // starting_width + row_inc
    for index, line in enumerate(data):
        data[index] = line * expand_count
    return data


def traverse_by(row_inc: int, column_inc: int) -> int:
    data = make_map(row_inc, column_inc)
    row, column, trees = 0, 0, 0
    while row < len(data):
        if data[row][column] == "#":
            trees += 1
        row += row_inc
        column += column_inc
    return trees


def main() -> None:
    mult = reduce(
        lambda a, b: a * b,
        [
            traverse_by(1, 1),
            traverse_by(1, 3),
            traverse_by(1, 5),
            traverse_by(1, 7),
            traverse_by(2, 1),
        ],
    )
    print(mult)


if __name__ == "__main__":
    main()
