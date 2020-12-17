# Thanks to /u/hugh_tc for their code helping me.


from collections import defaultdict
from copy import deepcopy
from dataclasses import dataclass
from itertools import product
from typing import List, Iterable, Set


@dataclass
class Point:

    coordinates: List[int]

    def neighbors(self) -> Iterable["Point"]:
        mods = product((-1, 0, 1), repeat=len(self.coordinates))
        for mod in mods:
            if 0 < sum(abs(axis) for axis in mod) <= len(self.coordinates):
                yield Point([c1 + c2 for c1, c2 in zip(self.coordinates, mod)])

    def __hash__(self) -> int:
        return hash(sum(self.coordinates))


def cycle(state: Set[Point]) -> Set[Point]:
    active_count = defaultdict(int, {p: 0 for p in state})
    for point in state:
        for n in point.neighbors():
            active_count[n] += 1
    next_state = deepcopy(state)
    for point, count in active_count.items():
        if point in state and count not in (2, 3):
            next_state.remove(point)
        elif point not in state and count == 3:
            next_state.add(point)
    return next_state


def main() -> None:
    state = set()
    with open("2020/day17/input.txt") as f:
        raw = [list(l.strip()) for l in f.readlines()]
        for y in range(len(raw)):
            for x in range(len(raw[0])):
                if raw[x][y] == "#":
                    state.add(Point([x, y, 0, 0]))
    for c in range(6):
        print(f"Performing cycle {c}")
        state = cycle(state)
    print(len(state))


if __name__ == "__main__":
    main()
