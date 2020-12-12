def main() -> None:
    with open("2018/day1/input.txt") as f:
        mods = [int(l) for l in f.readlines()]
    values = {0}
    val = 0
    while True:
        for mod in mods:
            val += mod
            if val in values:
                print(val)
                return
            values.add(val)


if __name__ == "__main__":
    main()
