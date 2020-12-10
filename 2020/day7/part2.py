from dataclasses import dataclass
import re
from typing import Dict, List, Tuple


BAG_NAME_REGEX = re.compile(r"(\w+ \w+) bags?")
BAG_NAME_COUNT_REGEX = re.compile(r"(\d+) (\w+ \w+) bags?")
SOURCE_BAG_NAME = "shiny gold"


@dataclass
class Bag:
    name: str
    children: List[Tuple[int, str]]


def burrow(bags: Dict[str, Bag], bag_name: str, count: int = 1) -> int:
    ret = 1
    for child_count, child_name in bags[bag_name].children:
        ret += child_count * burrow(bags, child_name, child_count)
    return ret


def main() -> None:
    with open("2020/day7/input.txt") as f:
        rules = [r.strip() for r in f.readlines()]
    bags = {}
    for rule in rules:
        bag_name = next(BAG_NAME_REGEX.finditer(rule)).group(1)
        children = BAG_NAME_COUNT_REGEX.findall(rule)
        bags[bag_name] = Bag(
            name=bag_name, children=[(int(c[0]), c[1]) for c in children]
        )
    ret = burrow(bags, SOURCE_BAG_NAME) - 1
    print(ret)


if __name__ == "__main__":
    main()
