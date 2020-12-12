def main() -> None:
    with open("2020/day6/input.txt") as f:
        lines = [line.strip() for line in f.readlines()]
    counts = []
    group = set()
    for line in lines:
        if not line:
            counts.append(len(group))
            group = set()
            continue
        group.update(list(line))
    counts.append(len(group))
    print(sum(counts))


if __name__ == "__main__":
    main()
