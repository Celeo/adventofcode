from dataclasses import dataclass
from typing import List, Set


@dataclass(eq=False, unsafe_hash=False)
class Point:

    x: int
    y: int
    steps: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def __eq__(self, o: object) -> bool:
        if not isinstance(o, Point):
            return False
        return self.x == o.x and self.y == o.y


def lay(directions: List[str]) -> Set[Point]:
    ret = set()
    x, y, steps = 0, 0, 1
    for direction in directions:
        compass = direction[0]
        amount = int(direction[1:])
        x_mod, x_mult, y_mod, y_mult = 0, 1, 0, 1
        if compass == "R":
            x_mod = amount
            x_mult = -1
        elif compass == "L":
            x_mod = amount
        elif compass == "U":
            y_mod = amount
        elif compass == "D":
            y_mod = amount
            y_mult = -1
        else:
            raise ValueError(compass)
        while x_mod > 0:
            x += x_mult
            ret.add(Point(x=x, y=y, steps=steps))
            x_mod -= 1
            steps += 1
        while y_mod > 0:
            y += y_mult
            ret.add(Point(x=x, y=y, steps=steps))
            y_mod -= 1
            steps += 1
    return ret


def main():
    with open("2019/day3/input.txt") as f:
        directions = [l.strip().split(",") for l in f.readlines()]
    first_points = lay(directions[0])
    second_points = lay(directions[1])

    least_by_distance = 2 ** 32
    least_by_steps = 2 ** 32

    for point in first_points:
        if point in second_points:
            least_by_distance = min(least_by_distance, abs(point.x) + abs(point.y))
            other = [p for p in second_points if p == point][0]
            least_by_steps = min(least_by_steps, point.steps + other.steps)

    print(least_by_distance)
    print(least_by_steps)


if __name__ == "__main__":
    main()
