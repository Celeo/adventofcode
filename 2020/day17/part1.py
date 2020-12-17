from dataclasses import dataclass
from itertools import product
from typing import Dict, List, Iterable


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


def cycle(state: Dict[Point, bool]) -> Dict[Point, bool]:
    next_state = {}
    for p in list(state.keys()):
        for n in p.neighbors():
            if n not in state:
                state[n] = False
    for p in state.keys():
        if state.get(p):
            active_count = 0
            for n in p.neighbors():
                if state.get(n):
                    active_count += 1
            if active_count in (2, 3):
                next_state[p] = True
        else:
            active_count = 0
            for n in p.neighbors():
                if state.get(n):
                    active_count += 1
            if active_count == 3:
                next_state[p] = True
    return next_state


def main() -> None:
    points = {}
    with open("2020/day17/input.txt") as f:
        raw = [list(l.strip()) for l in f.readlines()]
        for y in range(len(raw)):
            for x in range(len(raw[0])):
                points[Point([x, y, 0])] = raw[x][y] == "#"
    for _ in range(6):
        points = cycle(points)
    print(sum([e for e in points.values() if e]))


if __name__ == "__main__":
    main()
