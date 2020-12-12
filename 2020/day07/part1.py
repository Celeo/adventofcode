from dataclasses import dataclass
import re
from typing import Dict, List


BAG_NAME_REGEX = re.compile(r"(\w+ \w+) bags?")
SOURCE_BAG_NAME = "shiny gold"


@dataclass
class Bag:
    name: str
    children: List[str]


def climb(bags: Dict[str, Bag], bag_name: str) -> List[str]:
    ret = []
    for other in bags.values():
        if bag_name in other.children:
            ret.append(other.name)
            ret.extend(climb(bags, other.name))
    return ret


def main() -> None:
    with open("2020/day7/input.txt") as f:
        rules = [r.strip() for r in f.readlines()]
    bags = {}
    for rule in rules:
        parent, *children = BAG_NAME_REGEX.findall(rule)
        if "no other" in rule:
            children = []
        bags[parent] = Bag(name=parent, children=children)
    ret = climb(bags, SOURCE_BAG_NAME)
    print(len(set(ret)))


if __name__ == "__main__":
    main()
