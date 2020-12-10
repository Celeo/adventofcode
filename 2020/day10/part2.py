def main() -> None:
    with open("2020/day10/input.txt") as f:
        adapters = sorted([int(l) for l in f.readlines()])
        adapters.insert(0, 0)
    routes = {0: 1}
    for j in adapters[1:]:
        routes[j] = routes.get(j - 1, 0) + routes.get(j - 2, 0) + routes.get(j - 3, 0)
    print(routes[max(routes.keys())])


if __name__ == "__main__":
    main()
