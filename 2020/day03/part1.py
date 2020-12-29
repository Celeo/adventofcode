from typing import List


def get_prompt() -> List[str]:
    with open("2020/day03/input.txt") as f:
        return [line.strip() for line in f.readlines()]


def make_map(prompt: List[str]) -> List[str]:
    height = len(prompt)
    starting_width = len(prompt[0])
    needed_width = (height - 1) * 3 + 2
    expand_count = needed_width // starting_width + 1
    for index, line in enumerate(prompt):
        prompt[index] = line * expand_count
    return prompt


def main(prompt: List[str]) -> int:
    data = make_map(prompt)
    row, column, trees = 0, 0, 0
    while row < len(data):
        if data[row][column] == "#":
            trees += 1
        row += 1
        column += 3
    return trees


if __name__ == "__main__":
    print(main(get_prompt()))
