def main() -> None:
    with open("2020/day15/input.txt") as f:
        numbers = [int(l) for l in f.readline().split(",")]
    record = {}
    turn = 1
    last_said = None
    next_number = None
    while turn < 30_000_001:
        if numbers:
            next_number = numbers.pop(0)
        else:
            rls = record.get(last_said, -1)
            if rls != -1 and turn - rls > 0:
                next_number = turn - 1 - rls
            else:
                next_number = 0
        if last_said is not None:
            record[last_said] = turn - 1
        last_said = next_number
        turn += 1
    print(last_said)


if __name__ == "__main__":
    main()
