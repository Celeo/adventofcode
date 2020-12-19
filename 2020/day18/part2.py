# TODO unfinished

import re

SIMPLE_PAREN_REGEX = re.compile(r"(\([\d*+ ]+\))")
ADD_REGEX = re.compile(r"(\d+ \+ \d+)")


def add_paren(s: str) -> str:
    while True:
        repl = re.sub(ADD_REGEX, "(\\1)", s).replace("((", "(").replace("))", ")")
        if s == repl:
            return s
        s = repl


def evaluate(s: str) -> int:
    add_paren(s)
    while "(" in s:
        for group in SIMPLE_PAREN_REGEX.findall(s):
            inner = evaluate(group[1:-1])
            s = s.replace(group, str(inner))
    add_paren(s)

    ret, *symbols = s.split()
    ret = int(ret)
    for index in range(0, len(symbols), 2):
        operator = symbols[index]
        num = int(symbols[index + 1])
        if operator == "+":
            ret += num
        else:
            ret *= num
    return ret


def main() -> None:
    with open("2020/day18/input.txt") as f:
        expressions = [l.strip() for l in f.readlines()]
    # print(sum(map(evaluate, expressions)))

    values = []
    for expr in expressions:
        val = evaluate(expr)
        values.append(val)
        print(f"{expr} = {val}")


if __name__ == "__main__":
    main()
