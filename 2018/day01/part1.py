def main() -> None:
    with open("2018/day1/input.txt") as f:
        mods = [int(l) for l in f.readlines()]
    print(sum(mods))


if __name__ == "__main__":
    main()
