from dataclasses import dataclass
import re
from typing import List


RULE_REGEX = re.compile(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)")


@dataclass
class Rule:

    name: str
    low_1: int
    high_1: int
    low_2: int
    high_2: int

    @staticmethod
    def parse(line: str) -> "Rule":
        parts = RULE_REGEX.findall(line)[0]
        r = Rule(*map(int, (0,) + parts[1:]))  # type: ignore
        r.name = parts[0]
        return r


def invalid(rules: List[Rule], ticket: List[int]) -> int:
    count = 0
    for part in ticket:
        found = False
        for rule in rules:
            if rule.low_1 <= part <= rule.high_1 or rule.low_2 <= part <= rule.high_2:
                found = True
                break
        if not found:
            count += part
    return count


def main() -> None:
    rules = []
    your_ticket = []
    other_tickets = []
    with open("2020/day16/input.txt") as f:
        section = 0
        for line in f.readlines():
            line = line.strip()
            if not line:
                section += 1
                continue
            if section == 0:
                rules.append(Rule.parse(line))
            elif section == 1 and "," in line:
                your_ticket = list(map(int, line.split(",")))
            elif section == 2 and "," in line:
                other_tickets.append(list(map(int, line.split(","))))
    error_rate = sum([invalid(rules, t) for t in other_tickets])
    print(error_rate)


if __name__ == "__main__":
    main()
