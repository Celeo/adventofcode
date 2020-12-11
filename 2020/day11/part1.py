from copy import deepcopy
from typing import List


Grid = List[List[str]]


def get_adjacent(grid: Grid, x: int, y: int) -> List[str]:
    width, height = len(grid[0]), len(grid)
    points = (
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y - 1),
    )
    values = []
    for point in points:
        if 0 <= point[0] < height and 0 <= point[1] < width:
            values.append(grid[point[0]][point[1]])
    return values


def tick(grid: Grid) -> Grid:
    ret_map = deepcopy(grid)
    for x in range(len(grid)):
        for y in range(len(grid[x])):
            val = grid[x][y]
            adjacent = get_adjacent(grid, x, y)
            if val == "L" and "#" not in adjacent:
                ret_map[x][y] = "#"
            elif val == "#" and adjacent.count("#") >= 4:
                ret_map[x][y] = "L"
    return ret_map


def main() -> None:
    with open("2020/day11/input.txt") as f:
        grid = [list(l.strip()) for l in f.readlines()]
    current = deepcopy(grid)
    while True:
        nxt = tick(current)
        if nxt == current:
            break
        current = nxt
    occupied = 0
    for row in current:
        occupied += row.count("#")
    print(occupied)


if __name__ == "__main__":
    main()
