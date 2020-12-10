def fuelCost(mass: int) -> int:
    return (mass // 3) - 2


def main() -> None:
    with open("2019/day1/input.txt") as f:
        masses = [int(l) for l in f.readlines()]
    total = 0
    for mass in masses:
        fuel = fuelCost(mass)
        while fuel > 0:
            total += fuel
            fuel = fuelCost(fuel)
    print(total)


if __name__ == "__main__":
    main()
