from collections import Counter
from dataclasses import dataclass
import re
from typing import Tuple


Point = Tuple[int, int]
REGEX = re.compile(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d)")


@dataclass
class Claim:

    claim_id: int
    x: int
    y: int
    width: int
    height: int

    @staticmethod
    def from_str(s: str) -> "Claim":
        parts = REGEX.findall(s)[0]
        return Claim(*[int(p) for p in parts])

    def write(self, grid: Counter[Point]) -> None:
        for y in range(self.y, self.y + self.height):
            for x in range(self.x, self.x + self.width):
                grid[(x, y)] += 1


# TODO doesn't give the right answer :(
def main() -> None:
    with open("2018/day3/input.txt") as f:
        claims = [Claim.from_str(l.strip()) for l in f.readlines()]
    grid = Counter()
    for claim in claims:
        claim.write(grid)
    count = 0
    for _, freq in grid.items():
        if freq > 1:
            count += 1
    print(count)


if __name__ == "__main__":
    main()
