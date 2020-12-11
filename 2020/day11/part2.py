from copy import deepcopy
from enum import Enum
from typing import List, Optional


Grid = List[List[str]]


class Direction(Enum):

    Left = (-1, 0)
    Right = (1, 0)
    Up = (0, 1)
    Down = (0, -1)
    D1 = (-1, -1)
    D2 = (1, -1)
    D3 = (1, 1)
    D4 = (-1, 1)


def find_seat(grid: Grid, x: int, y: int, direction: Direction) -> Optional[str]:
    while True:
        x += direction.value[0]
        y += direction.value[1]
        if 0 <= x < len(grid) and 0 <= y < len(grid[0]):
            if grid[x][y] in ("L", "#"):
                return grid[x][y]
        else:
            return None


def get_adjacent(grid: Grid, x: int, y: int) -> List[str]:
    raw = [find_seat(grid, x, y, direction) for direction in Direction]
    return [r for r in raw if r]


def tick(grid: Grid) -> Grid:
    ret_map = deepcopy(grid)
    for x in range(len(grid)):
        for y in range(len(grid[x])):
            val = grid[x][y]
            adjacent = get_adjacent(grid, x, y)
            if val == "L" and "#" not in adjacent:
                ret_map[x][y] = "#"
            elif val == "#" and adjacent.count("#") >= 5:
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
