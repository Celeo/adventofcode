def fuelCost(mass: int) -> int:
    return (mass // 3) - 2


def main() -> None:
    with open("2019/day1/input.txt") as f:
        masses = [int(l) for l in f.readlines()]
    ret = sum([fuelCost(m) for m in masses])
    print(ret)


if __name__ == "__main__":
    main()
