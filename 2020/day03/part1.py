from typing import List


def make_map() -> List[str]:
    with open("2020/day3/input.txt") as f:
        data = [line.strip() for line in f.readlines()]
    height = len(data)
    starting_width = len(data[0])
    needed_width = (height - 1) * 3 + 2
    expand_count = needed_width // starting_width + 1
    for index, line in enumerate(data):
        data[index] = line * expand_count
    return data


def main() -> None:
    data = make_map()
    row, column, trees = 0, 0, 0
    while row < len(data):
        if data[row][column] == "#":
            trees += 1
        row += 1
        column += 3
    print(trees)


if __name__ == "__main__":
    main()
