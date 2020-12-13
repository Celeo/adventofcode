from itertools import count


def main() -> None:
    with open("2020/day13/input.txt") as f:
        earliest = int(f.readline())
        busses = [int(l) for l in f.readline().split(",") if l != "x"]
    for minute in count(earliest):
        for bus in busses:
            if minute % bus == 0:
                print(bus * (minute - earliest))
                return


if __name__ == "__main__":
    main()
