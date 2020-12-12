from dataclasses import dataclass
from enum import Enum


class Direction(Enum):

    North = 0
    South = 180
    East = 90
    West = 270


@dataclass
class Ship:

    x: int
    y: int
    direction = Direction.East

    def turn(self, action: str, value: int) -> None:
        if action == "L":
            value = -1 * value
        new_dir = self.direction.value + value
        while new_dir >= 360:
            new_dir -= 360
        while new_dir < 0:
            new_dir += 360
        self.direction = [d for d in Direction if d.value == new_dir][0]


def move(ship: Ship, instruction: str) -> None:
    action, value = instruction[0], int(instruction[1:])
    if action == "N":
        ship.y += value
    elif action == "S":
        ship.y -= value
    elif action == "E":
        ship.x -= value
    elif action == "W":
        ship.x += value
    elif action in ("L", "R"):
        ship.turn(action, value)
    elif action == "F":
        move(ship, f"{ship.direction.name[0]}{value}")


def main() -> None:
    with open("2020/day12/input.txt") as f:
        instructions = [l.strip() for l in f.readlines()]
    ship = Ship(x=0, y=0)
    for instruction in instructions:
        move(ship, instruction)
    print(abs(ship.x) + abs(ship.y))


if __name__ == "__main__":
    main()
