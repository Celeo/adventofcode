def main() -> None:
    with open("2020/day10/input.txt") as f:
        adapters = sorted([int(l) for l in f.readlines()])
        adapters.append(max(adapters) + 3)
    diffs = {1: 0, 3: 0}
    for index, val in enumerate(adapters):
        last_val = adapters[index - 1] if index > 0 else 0
        delta = val - last_val
        diffs[delta] += 1
    print(diffs[1] * diffs[3])


if __name__ == "__main__":
    main()
