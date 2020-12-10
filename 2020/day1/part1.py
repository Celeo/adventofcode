from itertools import combinations


def main():
    with open("2020/day1/input.txt") as f:
        data = combinations([int(line) for line in f.readlines()], 2)
    for pair in data:
        if pair[0] + pair[1] == 2020:
            print(pair[0] * pair[1])
            return


if __name__ == "__main__":
    main()
