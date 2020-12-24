# TODO unfinished

from typing import Dict, Optional, Tuple


def log(message: str, depth: int) -> None:
    print(f"{'>' * depth} {message}")


def message_valid(
    message: str,
    rules: Dict[int, str],
    rule_num: int = 0,
    rule_overide: Optional[str] = None,
    depth: int = 1,
) -> Tuple[str, bool]:
    if rule_overide:
        rule = rule_overide
        log(f'Checking "{message} against override rule: {rule}', depth)
    else:
        rule = rules[rule_num]
        log(f'Checking "{message}" against rule {rule_num}: {rule}', depth)

    if len(rule) == 1 and rule.isalpha():
        if message[0] == rule:
            log("Letter check passed", depth)
            return (message[1:], True)
        log(f"Letter check failed, wanted {rule}, got {message[0]}", depth)
        return (message, False)
    elif "|" in rule:
        left, right = [o.strip() for o in rule.split("|")]
        log("Checking left rule", depth)
        left_rest, left_ok = message_valid(message, rules, -1, left, depth=depth + 1)
        if len(left_rest) == 0 and left_ok:
            log("Left side check successful", depth)
            return (left_rest, left_ok)
        log("Checking right rule", depth)
        right_rest, right_ok = message_valid(message, rules, -1, right, depth=depth + 1)
        if len(right_rest) == 0 and right_ok:
            log("Right side check successful", depth)
            return (right_rest, right_ok)
        log("Neither left nor right were successful", depth)
        return (message, False)
    else:
        composite = map(int, rule.split(" "))
        msg = message
        for inner in composite:
            msg, ok = message_valid(msg, rules, inner, depth=depth + 1)
            if not ok:
                log("Rule part failed", depth)
                return (msg, False)
        return (msg, True)


def main() -> None:
    rules = {}
    messages = []
    section = 0
    with open("2020/day19/input.txt") as f:
        for line in f.readlines():
            line = line.strip()
            if not line:
                section += 1
                continue
            if section == 0:
                num, *rest = line.split(": ")
                rules[int(num)] = rest[0].replace('"', "")
            else:
                messages.append(line)
    valid = []
    for msg in messages:
        rest, ok = message_valid(msg, rules)
        if len(rest) == 0 and ok:
            valid.append(msg)
    print(len(valid))


if __name__ == "__main__":
    # Positive
    assert message_valid("a", {0: "a"}) == ("", True)
    assert message_valid("a", {}, rule_overide="a") == ("", True)
    assert message_valid("ab", {0: "a"}) == ("b", True)
    assert message_valid("ab", {0: "1 2", 1: "a", 2: "b"}) == ("", True)
    assert message_valid("abc", {0: "1 2", 1: "a", 2: "b"}) == ("c", True)
    assert message_valid("a", {0: "1 | 2", 1: "b", 2: "a"}) == ("", True)
    assert message_valid("ab", {0: "1 2 | 2 1", 1: "b", 2: "a"}) == ("", True)
    assert message_valid("abc", {0: "1 2 | 2 1", 1: "b", 2: "a"}) == (
        "c",
        True,
    )  # FIXME I think this should pass

    # Negative
    assert message_valid("a", {0: "b"}) == ("a", False)

    # main()
