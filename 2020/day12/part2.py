from dataclasses import dataclass


@dataclass
class Point:

    x: int
    y: int


def move(ship: Point, waypoint: Point, instruction: str) -> None:
    action, value = instruction[0], int(instruction[1:])
    if action == "N":
        waypoint.y += value
    elif action == "S":
        waypoint.y -= value
    elif action == "E":
        waypoint.x += value
    elif action == "W":
        waypoint.x -= value
    elif action == "L":
        while value > 0:
            waypoint.x, waypoint.y = -1 * waypoint.y, waypoint.x
            value -= 90
    elif action == "R":
        while value > 0:
            waypoint.x, waypoint.y = waypoint.y, -1 * waypoint.x
            value -= 90
    elif action == "F":
        ship.x += waypoint.x * value
        ship.y += waypoint.y * value


def main() -> None:
    with open("2020/day12/input.txt") as f:
        instructions = [l.strip() for l in f.readlines()]
    ship = Point(x=0, y=0)
    waypoint = Point(x=10, y=1)
    for instruction in instructions:
        move(ship, waypoint, instruction)
    print(abs(ship.x) + abs(ship.y))


if __name__ == "__main__":
    main()
