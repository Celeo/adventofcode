from collections import defaultdict
from dataclasses import dataclass
from functools import reduce
import re
from typing import Dict, List


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


def determine_fields(rules: List[Rule], tickets: List[List[int]]) -> Dict[int, str]:
    possible_mappings = defaultdict(set)
    for rule in rules:
        for index in range(len(tickets[0])):
            if all(
                [
                    rule.low_1 <= t[index] <= rule.high_1
                    or rule.low_2 <= t[index] <= rule.high_2
                    for t in tickets
                ]
            ):
                possible_mappings[index].add(rule.name)

    assigned = {}
    pm = sorted(possible_mappings.items(), key=lambda pair: len(pair[1]))
    for v in pm:
        index = v[0]
        possible_fields = v[1]
        taken_fields = list(assigned.values())
        filtered_fields = [
            field for field in possible_fields if field not in taken_fields
        ]
        assigned[index] = filtered_fields[0]
    return assigned


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
                your_ticket = [int(l) for l in line.split(",")]
            elif section == 2 and "," in line:
                other_tickets.append([int(l) for l in line.split(",")])
    valid_tickets = [t for t in other_tickets if not invalid(rules, t)]
    fields = determine_fields(rules, valid_tickets)
    your_ticket_fields = []
    for index, name in fields.items():
        if name.startswith("departure"):
            your_ticket_fields.append(your_ticket[index])
    ans = reduce(lambda a, b: a * b, your_ticket_fields)
    print(ans)


if __name__ == "__main__":
    main()
