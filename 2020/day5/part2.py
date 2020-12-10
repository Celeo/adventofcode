from typing import Tuple


ROW_COUNT = 128
COLUMN_COUNT = 8


def find_position(pos: str) -> Tuple[int, int, int]:
    rows = list(range(ROW_COUNT))
    for loc in pos[:7]:
        midpoint = len(rows) // 2
        front, back = rows[:midpoint], rows[midpoint:]
        if loc == "F":
            rows = front
        else:
            rows = back

    columns = list(range(COLUMN_COUNT))
    for loc in pos[7:]:
        midpoint = len(columns) // 2
        front, back = columns[:midpoint], columns[midpoint:]
        if loc == "L":
            columns = front
        else:
            columns = back

    return (rows[0], columns[0], rows[0] * 8 + columns[0])


def main() -> None:
    with open("2020/day5/input.txt") as f:
        lines = [line.strip() for line in f.readlines()]
    seat_ids = []
    for line in lines:
        _, _, seat_id = find_position(line)
        seat_ids.append(seat_id)
    seat_ids.sort()
    for pos in range(seat_ids[0], seat_ids[-1]):
        if pos not in seat_ids and (pos - 1) in seat_ids and (pos + 1) in seat_ids:
            print(pos)
            return


if __name__ == "__main__":
    main()
